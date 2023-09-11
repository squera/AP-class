++ The (broken) MP Specifications
===========================================================
The purpose of this document is to define the specs of the
market protocol, which involves these notions:
- goods
- goods metadata
- markets
The document then also covers
- code testing
for the aforementioned notions and concludes with a
summary of the files that make the Market Protocol.
This document also refers to the trader, the other entity
you have to program, as described in the Project
Description.

++++ Overall Description
-----------------------------------------------------------
The trader acquires some quantity of `goods` from each
market according to the `ask` and `sell` price that a
good has for a specific market.
The same good can exist on different markets with different
`quantity`, a different `ask` price and a different `sell`
price.
When a trader has a good, the good does not have an `ask`
or `sell` price.


+++ Good, GoodKind and GoodError
===========================================================
A central entity to the Market Protocol is that of goods,
that exist in markets and that are traded by the trader
bots.

++++ Good Description
-----------------------------------------------------------
A good is a struct with a `name` : `GoodKind`, and a
`quantity` : `int`. Both fields are private.

++++ GoodKind Description
-----------------------------------------------------------
`GoodKind` is an Struct telling all the possible kinds of 			
goods that can be created, and so it contains:
EUR, USD, YEN, YUAN

++++ GoodError Description
-----------------------------------------------------------
`GoodError` is an Enum telling all the possible kinds of
errors that can be returned by good-related functions.

++++ Goods Creation
-----------------------------------------------------------
There are three ways to create a `Good`:
from a `String`, from a GoodKind, and a default good.

`new` : String nm -> Good													
depending on the value of `nm`, pick the right
`GoodKind` to create the related good

`new_default` : -> Good
create EUR

`new_from_enum` : &GoodKind gk -> Good
create a `Good`

All three functions set the quantity to some
`STARTING_QTY`
that is defined externally, in the constants file
consts.rs

++++ Goods Functionality
-----------------------------------------------------------
A Good offers the following public functions with the
following signatures

`split` : &mut self, i32 q -> Result<Good>
if `self.quantity` is greater than `q`
decrement `self.quantity` by `1`
return a new good with `quantity` = `q`
otherwise return `NotEnoughQty`

`get_q` : &mut self -> i32 										
returns `self.quantity`

`get_kind` : &self -> GoodKind
returns the kind of the good

`is_qty_enough` : &self, i32 q -> bool

`to_string` : &self -> String
returns a string describing the `Good`

`merge` : &self, &mut Good g -> Result<(),GoodError> 	
let the quantity of `g` be `q`
sets the quantity of `g` to `0`
adds `q` to `self.quantity`
unless the kinds of goods differ, in which case,
this returns `DifferentKindsOfGood`

`iseq` : &self, &Good g -> String
tests if self and `g` have the same kind

`is_default` : &self -> bool
tests if self is `EUR`


+++ Good Metadata
===========================================================
Markets must maintain a list of goods and metadata for
each good as well.
The metadata needs to keep track of
- the current state of the good in the market
- the current trading price for that good in the market
both for selling and for buying
The state of a good needs to record what state the good
is in. A good can be in no state, it can have received an
offer for a buy for some quantity at some price, or it
can have received an offer for a sale for some quantity at
some price.

The current trading price indicates the price for selling
and for buying a good in the market.

Please take a look at the market buying and selling
protocols below in order to have a better idea of how the
Good Metadata is used by a market.

The Good Metadata is, essentially, local to the market,
so any communication between trader and market does not
handle any explicit details of Good Metadata.
This is the reason why this section is left unspecified
(that is, we do not say that Good Metadata is a struct,
or an enum, nor define its impl).
In fact, the metadata should be only used locally by a
market, and your metadata is used only by your market,
so there is no need to define this further than what
already stated above.


++++ Market
===========================================================
A market has a name and collection of goods, each with
their own good metadata.
The main business of a market is providing a buy and a sell
interface, alongside other functionalities.

The list of goods is the same for all markets
(i.e., for all groups).

+++++ Market Creation
-----------------------------------------------------------
In order to create a market, you give it your group's name
and then it allocates empty collections for all the rest,
that is, for its goods and each good metadata.
The amount and kinds of goods to be allocated upon market
creation is for you to decide.

A market can be created on debug mode.
In that case, the kind of goods it has and their amounts
is fixed.

+++++ Market Functionality
-----------------------------------------------------------
Markets provide some functionalities beyond the buy and
sell protocols:

`get_market_name` : &self -> &mut str
returns the market `name`

`get_budget` : &self -> Good
returns the good `EUR` of the market

`get_goods` : &mut self -> Vec<&Good>

+++++ Trader Buying from Market Protocol
-----------------------------------------------------------
In order to buy from the market, first one has to lock a
good. Locking a good at a certain price and for a certain
quantity gets registered in the good's status.

`lock_trader_buy_from_market`: &self, &GoodKind g, bool b, 	
i32 p, i32 q, String d -> Result<(), MarketError>
a buy can fail if the good has already been locked on
a different state,
or if there is not enough quantity that is being
locked in an offer, or if the price is not enough for
the offer.
Otherwise, the locking succeeds and the market
registers (via the market-local Good Metadata) the
fact that quantity `q` of good `g` is to be bought for
price `p`. The market locks the good for buying.
For the offer to be enough, it must be at least as much
as the current good price, plus a certain % that is
not random.

***********************************************************
META-LEVEL COMMENT
In an attempt to clarify what it means for you to fix
these specs, consider this: who decides the above % ?
This is an example of underspecification.
Is it ok for this % to be left arbitrary?
Do you think this will cause issues as it is ?
Or should you (as the WG) impose a constraint, or
perhaps a fixed % value to avoid possibles issues?
Under (or over) specification can be problematic, and
it is your duty as WG to rid this document of any
problematic under (or over) specification.

***********************************************************

`trader_buy_from_market`: &mut self, &GoodKind g, Good cash
-> Result<Good>
a buy can fail if: the market does not have that good,
the good's state is not locked-in for buying, the cash
is not enough to cover the qty x price locked in the
good's state.
Othewise, buying succeeds and the returned kind is the
same that is passed as `g`, the quantity is the one
that was locked in the state of the good. Afterwards,
the good's state gets reset, the cash gets added to
the market cash.
Finally, the price of the good needs to be updated to
the offer price.

+++++ Trader Selling to Market Protocol
-----------------------------------------------------------
In order to sell to the market, first one has to lock a
good at a certain price, for a certain quantity.

`lock_trader_sell_to_market`: &mut self, g: GoodKind,
qty: mut i32, price: i32 -> Result<()>
The locking of a sale can fail if the market does not
have enough `EUR`, or if the good to be locked is not in
the right state (i.e., not ready to be sold, perhaps
already locked for sale or for buy), if the offer from
the trader is not high enough.
Otherwise, the locking succeeds.

`trader_sell_to_market`: &mut self, good: Good ->
Result<Good, MarketError>
The selling of a good can fail if the good to be sold
is in the wrong state, or if the quantity of the good
`good` is different than the quantity in the good
offer metadata of the market for that good, or if the
market has less money than needed to pay for the
quantity of the good.
Otherwise, selling succeeds, the market adds `good` to
its list, merging it with any other quantity of good
of the same kind it already has, and returns `EUR` goods
in quantity equal to the price multiplied by the
supplied quantity. The price for the sold good is
then decreased in a manner analogous to how it was
increased during the buy.

+++++ Buying and Selling Default Good: EUR
-----------------------------------------------------------
When buying and selling EUR (the default good used to
pay for any other good), do not increase or decrease
its metadata price. Buying and selling EURs is always done
at a 1:1 ratio.

+++++ Market Issues
-----------------------------------------------------------
The market can become unusable if the goods are all locked
but nobody is buying. We call this: being "deadlocked".
The market must have an internal mechanism to prevent
being deadlocked, which is described below:


++++ Market Errors
-----------------------------------------------------------
The market can throw a number of errors, as captured by the
`MarketError` enum.

Also, Market Errors include a generic error that allows
easy pretty printing: GeneralError(String).


++++ Code Testing
===========================================================
These specs also dictate how the testing of the common
code is to be run. This paragraph tells which modules need
to be tested.

It is the WG's duty to spell out which unit tests are to
be carried out and the goal of each unit test function.
Moreover, you should arrange the workload of coding
these tests amongst the groups.

+++++ Goods Tests
-----------------------------------------------------------
For the `Good` to be considered valid according to these
specs, there must also be unit tests for it.
Since the good is a single entity shared by all groups,
these tests can be placed in the same file as `Good`.
These tests can be found at:

The unit tests for `Good` are:

+++++ Market Tests
-----------------------------------------------------------
The unit tests file
`MarketsTest`
needs to run correctly in order for a market to be
considered valid according to these specs.
You need to run these test on your market only.

These tests can be found at:

++ Code and Usage
===========================================================
All common code needs to be commented, possibly with
reference to this document and indicate the author of the
comment.

Ultimately, there needs to be a number of shared files
in the common repo that everyone must be able to download
and build upon: 	
- Good: containing the Good struct, its Impl
and its tests
- GoodKind, containing the GoodKind Enum
- GoodError, containing the GoodError Enum
- MarketError, containing the MarketError Enum
- MarketsTest, containing the unit tests for anything
Market-related
- one market file per group
