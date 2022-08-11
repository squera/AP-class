++ Project Description
====================================================================
This document describes the class project.

From a high-level perspective,
you have to program a trader bot that reads data
from multiple currency markets and performs basic trading
on them following a trading strategy that you have to
implement. The bot displays its strategy and its actions
on the markets, reporting its earnings and its losses.
The trader bot needs to interact not just with your market,
but also with markets written by other groups.
That is, you'll download their markets and
run your code with their market instances.
To ensure each group's trader can interact with each group's
market, we need to set up a Market Protocol (MP), which
defines all operations that can be done on markets and on
their currencies (called goods in the Market Protocol).
We provide a **faulty** definition of the specs of the protocol
that you  have to fix, following the process of a W3C-like
organisation (described below).

This document contains the following information
- a description of the Project Components
- a description of W3C-like working groups and all its components
- a description of how each group's trader will use other
  groups' markets
- a summary of project dates and exam details.

The faulty Market Protocol is described in a separate file.


+++ Project Components
====================================================================
There are some key components of the project that
each group must implement.
1. one currency market
2. one trader bot with its own strategy
4. one trader history visualiser
   We now describe them in more detail and follow up with a
   description of how they will be evaluated.

++++ The Individual Markets
--------------------------------------------------------------------
Each group is named after a single market (e.g., Milano,
New York, Amsterdam, Tokyo, Singapore ...).
Each group implements a single currency market with their name.
Details of the behaviour of the market are in the Market
Protocol file.

++++ Trader Functionality
--------------------------------------------------------------------
The bot must interact with at least 3 markets, buying and
selling goods according to a strategy. It must also record
all of its transactions in order for them to be displayed.

++++ Strategy Displayer
--------------------------------------------------------------------
The trader must log its actions accurately and record
them in written form.
A .txt describing the log is the bare minimum, a .pdf or
a more complex displayer is recommended.

++++ Evaluation
--------------------------------------------------------------------
The project is evaluated on different aspects:
correctness, functionality with more or less markets,
fancyness of strategy, fancyness of logging.


+++ W3C-like Working Groups
====================================================================
During the first lecture, the class will split up in groups
of 3/4 people, preferably 4.
Each group elects a leader (WGL): that person is responsible
of voicing the group's concerns when the market protocol
working group meets.
The class also elects a coordinator (WGC), a person responsible
of making the various market protocol meetings productive.

The assembly of all WGL plus the WGC is called the
Working Group (WG)
The WG will have meetings to correct the faulty Market Protocol
and draft a new, sound one.
During each meeting, the responsibility of the WG
is to discuss different aspects of the MP protocol, fix
them and reach a consensus of what the protocol must do and how.

The WG is registered in piazza, the list of all WG members
is listed in a pinned post.

++++ Working Group Leaders (WGL)
--------------------------------------------------------------------
Each group elects its leader, who will represent the group
at each WG meeting. They are responsible for voting changes
to the protocol, and they are the only ones that can vote.

++++ Working Group Coordinator (WGC)
--------------------------------------------------------------------
The class must elect a working group Coordinator.
The Coordinator is responsible of drafting each new
version of the protocol and to submit the shared code to the
repo. Despite the fact that they come from a group, they must
act super-partes.
This role requires some extra work and this is reflected
in the grading with a bonus the total result.

++++ Working Group Process
--------------------------------------------------------------------
Ideally, once the project description is out and the faulty
market protocol is known, each group starts implementing it
and discovers faults in the specifications.
By calling out WG meetings, faults in the specifications are
corrected with suggestions from each group.
Also, as you will see, there are some shared functionalities
(goods, i.e., currencies) whose implementation also needs to
be provided for all groups' markets to use.
The WG must also rule how these functionalities are coded.

The WG is advised to meet regularly and meet frequently, at
least in the beginning of the semester, to ensure smooth
progress.
Ultimately, the WG will produce an alpha version and then
a beta version, i.e., a final version of the protocol that
will be frozen.

+++++ Freezing the Protocol
--------------------------------------------------------------------
Approaching the end of the semester the MP becomes *frozen*
(see the dates below). That is, no further modifications
are possible (except for typos and the like)
unless there are serious pitfalls in its design.
In order to unfreeze the MP, the
WGC must have the consensus of the WGL (>=51%) and they
must contact me. If the request sounds reasonable, the
protocol is unfrozen for 1 meeting, when it is fixed
and then frozen again.
Note that it is in your best interest to not unfreeze
a protocol since changes to the MP means changes to your code.
Also, too many unfreezes will reflect poorly on the
evaluation of the WGL and of the WGC. However, you have
all the time and capabilities to reach a stable protocol
in time, so that no unfreezing is necessary.


+++ Sharing Code
====================================================================
As mentioned, each group's traders need to use markets from
other groups too.
The logical connection between each group's trader and the
markets is what the market protocol is there for.
However, this kind of interacation also requires some
technical set up which is described here.
Furthermore, here we also describe how the usage of other
group's markets is regulated between all groups.

++++ Class Code Repository
--------------------------------------------------------------------
TODO: describe code repo
TODO: git-like?

++++ Software Faire
--------------------------------------------------------------------
In order to know which markets your group will use, towards
the end of the semester we'll have a Faire, where each
group will showcase their market implementation and where
each group will decide which other groups' markets they
are going to use.

+++++ Committing to Markets
--------------------------------------------------------------------
During the fair, each group must commit to use
three (3)
markets from other groups.
By the end of the fair, we will ask each group's choices.
These choices are final, and in the final project
evaluation, your trader will have to run with these groups'
markets.

The group that sells the most of their markets will
impact their grading, members of that group will receive a
bonus to their grade.
The group that sells the second most of their markets
will have a smaller bonus to their grade.


+++ Project Organisation and Exam Details
====================================================================
The project has some fixed dates that are central and that
dictate how other organisational matters evolve.

Specifically:
- the groups are formed                           1st class
- the MP protocol is released                     7th class
- the WG is formed, the WGL and WGC are elected   7th class
- each group starts working
- the protocol is frozen                    (max) 21st class
- the market fair happens                         23rd class
- the commitments are registered                  23rd class
- the exam happens                     
  TODO: dates. Do not clash with popl week.
  Do wed or thu ?

The MP protocol is not released sooner because it would make
little sense, since it talks about technical Rust notions
that you don't know yet.

++++ Exams
--------------------------------------------------------------------
A typical exam is the showcase of the functionality of your
project and a discussion of its parts.
During a typical exam day, groups will register for a
timeslot on piazza.
Each timeslot is 45 minutes, which include
5 minutes to enter, set up the showcase
30 minutes for showcase and questions
5 minutes to exit
5 minutes for me to write an evaluation
The examination is held in my office:

Failure to show up to a booked slot, without proper and sound notice
is ground for terrible punishment.


++++ Ethics Considerations
====================================================================
Once you're done, sit down and think about the ethical
considerations of your code. Similar pieces of code exist
out there. What kind of social challenges do they face?
Is it morally right to produce such software? Why? Why not?


