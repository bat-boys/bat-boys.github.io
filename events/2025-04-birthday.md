# Coral ring / the thirtyfive event

- There are coral rings inside areas (e.g. `Giant magenta coral ring with four slots <white glow>`)
- They all need four items to open an entrance to a boss
- Recipes  (e.g. `Folded circle of magenta paper`) tell which four items are needed
  for which ring are scattered around the world as well (mob drops, syntaxes, etc.)
- Each ring remains open until boot, can be opened again with the same items
- Entering a ring is worth 1 event point, killing a boss is worth 2 points
- Arelium adventurers' guild commands `la plaque`
  - `35info` for event progress
  - `35clue` for hint about where to look for a recipe

[Google sheet for rings and items](https://tinyurl.com/shinaevent35)

## Prizes

Prize shop at `Arelium (Arelium bazaar) [99,23]` (near Ugly Joe's restaurant)

```
la sign

Syntax: 35prize list
        35prize view <number>
        35prize buy <number>
Prizes are not refundable until one year has passed.
Please be very specific about which items you purchase.

35prize list

Prizes available for purchase:
1:   con_hpr_spr cloud                                                 (30)
2:   dex_con_hpr cloud                                                 (30)
3:   dex_con_spr cloud                                                 (30)
4:   dex_hpr_spr cloud                                                 (30)
5:   int_con_hpr cloud                                                 (30)
6:   int_con_spr cloud                                                 (30)
7:   int_dex_con cloud                                                 (30)
8:   int_dex_hpr cloud                                                 (30)
9:   int_dex_spr cloud                                                 (30)
10:  int_hpr_spr cloud                                                 (30)
11:  int_wis_con cloud                                                 (30)
12:  int_wis_dex cloud                                                 (30)
13:  int_wis_hpr cloud                                                 (30)
14:  int_wis_spr cloud                                                 (30)
15:  str_con_hpr cloud                                                 (30)
16:  str_con_spr cloud                                                 (30)
17:  str_dex_con cloud                                                 (30)
18:  str_dex_hpr cloud                                                 (30)
19:  str_dex_spr cloud                                                 (30)
20:  str_hpr_spr cloud                                                 (30)
21:  str_int_con cloud                                                 (30)
22:  str_int_dex cloud                                                 (30)
23:  str_int_hpr cloud                                                 (30)
24:  str_int_spr cloud                                                 (30)
25:  str_int_wis cloud                                                 (30)
26:  str_wis_con cloud                                                 (30)
27:  str_wis_dex cloud                                                 (30)
28:  str_wis_hpr cloud                                                 (30)
29:  str_wis_spr cloud                                                 (30)
30:  wis_con_hpr cloud                                                 (30)
31:  wis_con_spr cloud                                                 (30)
32:  wis_dex_con cloud                                                 (30)
33:  wis_dex_hpr cloud                                                 (30)
34:  wis_dex_spr cloud                                                 (30)
35:  wis_hpr_spr cloud                                                 (30)
101: Cleanser that turns eq back into a cloud                          (1)
102: Polisher that replaces a major stat with a minor one              (1)
103: Pcity basket that gives +stat boosting streamers                  (30)
104: Book that teaches the candymaking skill from Halloween 2019       (30)


35prize view 101

A powerful mystic solvent, it can remove the essence of a Thirtyfive event eq
and revert it back to a cloud, allowing its use in a different armour or
weapon. This will destroy the current eq at the same time.
Syntax: cleanse <35 eq> with <this>
It is labeled as 'Formula 35'.
It costs 1 points.

35prize view 102

A cloth treated with mystical granules for smoothing and refining
enchantments. Can be used to swap one major stat of a prize item with a minor
one.
Syntax: polish <35 eq>
This will tell you the list of major stats it has.
        polish list
Lists the minor stats you can use.
        polish <35 eq> swap <stat to remove> with <stat to replace>
Removes any previous polish on the Thirtyfive Event prize, if any.
Then, replaces one of its three major stats with the minor stat you choose.
This uses up the cloth.
It costs 1 points.

polish list

The minor stats you can add are:
  hit
  dam
  def [for avoid]
  cha
  phr [for resistance to physical damage]
  enr [for resistance to nonphysical damage]
  inf [for adding infravision]
  led [for adding summon resistance]
  spd [for adding damage to many spells]
  sph [for adding healing to many spells]
You may only replace one major stat with one minor stat.

35prize view 103

This small container holds a number of party poppers left over from the
Thirtyfive Event. You could take some, use them yourself or share them, and
get a burst of festive feelings, adding one of the major stats for a short
time.
Streamer bonuses do not stack with each other.
Syntax: streamer <colour>
It costs 30 points.

35prize view 104

Reading this book will teach you the skill 'candymaking' at exactly the same
skill percent as you currently have 'cooking' trained. The book will be
destroyed on use.
It costs 30 points.
```

## News post

```
news display inform 1022
```

[Post on Batmud website](https://www.bat.org/forums?a=view_post&postid=60332&group_name=inform&page=1)

This year's April Fool's Event is not just a celebration of BatMUD's 35th
anniversary, but happens to also be the 35th seasonal event I've coded, going
all the way back to the Exomorph Event first run in 2006.

Spread around BatMUD are 35 coral rings, each of which require the sacrifice
(destruction) of four items of various types. The rings do not tell you which
items are needed, only if you chose correctly. You could try to brute force
the rings by slamming thousands and thousands of items into them...good luck,
there are 87 trillion combinations. Or, you could find the combinations,
written on magenta circles of paper, _also_ scattered around BatMUD.

Behind each ring is a new room and a new monster for your E% and holz
purposes. The monsters will vary in difficulty and give an indication when
you've done enough effort to get credit for the kill. In a bit of a change
this year, the bosses are not as tough as they've been in the past, and they
also reset. Once a ring unlocks a boss, it should stay open until reboot.

Event points are earned in two ways. You get one point for entering a new
room, and two points for a boss kill. That's three points per ring, 35 rings,
105 maximum potential points.

Event prizes will be made available for purchase at the end of the Event, but
each will have +greatly in three stats, such as str/int/wis or dex/con/hpr.
Each of the 35 bosses unlocks a new trio of stats when slain by _anyone_. You
don't need to personally beat the str/int/wis boss to buy str/int/wis eq, but
it does have to be unlocked and die at least once.

In other words, MUDwide teamwork is pretty much essential. No individual
mortal is going to discover all the combinations and unlock all the rings on
their own.

If you don't like the classic stats of str/int/wis/dex/con/hpr/spr, it will be
possible to change things. Each eq costs 30 points, so you'll likely have a
few points left over. For 1 point, you can buy an item that replaces one of
the eq's stats with a secondary stat (such as hit or avoid) at +slightly
level. You can buy multiple such cleaners and chest them for future reincs.
The Event eq has the usual features (only the buyer may use, eq is
invulnerable) and, for this special Event, is _permanent_. That's right, the
usual one-year lifespan for seasonal Event eq is not going to apply just this
once.

The Thirtyfive Event will run for 35 days. So get hunting!

Shinarae "Happy 35th Anniversary BatMUD" Lluminus


```
news display updates 1565
```

[Post on Batmud website](https://www.bat.org/forums?a=view_post&postid=60338&group_name=updates&page=1)

Starting at reboot, you should be able to use the sign in the Adventurer's
Guild to test the prizes for length of that boot. Simply pick a combo of stats
that's been unlocked, and some useless armour or weapon to shove the stats
into. Whatever item you make will be locked to you, and nosave, and you get
one per day.

Let me know if the syntaxes don't work or anything else goes wrong.

Shinarae "int_wis_cha" Lluminus

```
news display updates 1566
```

After hearing comments and concerns from multiple mortals, I've made some
changes to the prizes of the Event.

1) Starting one year after the event, the eq prizes will become single-wear
only. They will still remain at full stats, still remain adjustable, and still
remain indestructible. You just won't be allowed to wear two of them at once
starting May 2026. I'm sorry that this seems like such a bait-and-switch, but
the questions about stat balance were just too many and too serious to ignore.
Perhaps this is an excuse to collect different prizes for different reincs.

2) So, let's add some good news. The "slightly" powers you can replace one
stat with already includes +hit, +dam, +avoid and +infra. Joining them are
+lead ring effect, +spell damage, +spell healing, +physical res, and +energy
resist. This should give you a few more options to play with, especially
casters.

3) Instead of buying a piece of eq, you'll also have the option of buying a
basket of streamers, a boltable pcity item. Once per reboot, you can collect 7
party poppers, one for each str/int/wis/dex/con/cha/regen which when used add
+divine to that stat for 30 minutes. These buffs will not stack (you can only
use one at a time) but the items are tradable. 

I am open to suggestions to other non-eq items. This would be the time to ask.

4) You will be allowed to refund a piece of Event eq later, even when a year
has passed, for other non-eq items (such as the streamer basket). Meaning, you
could use three items for a year, then trim down to one when that stops being
possible.

5) For the rest of the event, you may receive celebratory booze at the Event
sign. That booze tests the party popper stat buffs, so give them a try. Like
the poppers the booze is tradable but does not stack.

Shinarae "pop the cork" Lluminus
