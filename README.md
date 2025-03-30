# Convoy

Convoy is a board game I am creating as a rust library. It is a turn-based strategy game inspired by advance wars.

## Overview

Players spend money to buy units who take control of the board and capture towns to increase their income, which they
then spend on more units to take board control from their opponent.

### Units

#### Artillery

Statistics:

+ Cost: 4
+ Power: 2
+ Range: 2-3
+ Speed 2

The artillery unit represents

#### Convoy

Statistics:

+ Cost: 3
+ Power: 0
+ Range: 0
+ Speed: 3

The convoy unit represents a collection of administrative and support staff, supply handlers, and transportation. They
define how

#### Infantry

Statistics:

+ Cost: 2
+ Power: 2
+ Range: 1
+ Speed: 2

The infantry unit represents company sized detachment of conscripted troops. They require the support of a logistics
system to stay fed, happy, and in good fighting shape. While their size can make them slow to mobilize, they are highly
effective in direct combat and can hold the line against enemies. The infantry provide structure and stability to your
command, able to act as both spear and shield throughout tactical engagements.

#### Recon

Statistics:

+ Cost: 4
+ Power: 1
+ Range: 1
+ Speed: 4

The recon unit represents a small elite mobile reconnaissance team. Their few numbers render them a non-threat to any
notable armed force but grants them the manoeuvrability to flee from a battle. They are the only unit capable of
supplying themselves, allowing them to infiltrate enemy controlled territory and sabotage critical logistics
infrastructure. The recon is a wildcard that can force you to mind your flanks and divert critical troops from the
front-lines.

## Rules

### Concepts

#### Units

#### Moving Units

#### Purchasing Units

#### Towns

#### Logistic Network

#### Battles

### Setup

### Gameplay

The player that begins is at the discretion of the players. Each turn is split into three phases.

#### Upkeep

- Gain one funding for each [town](#towns) tile you control connected to your logistics network
- All exhausted units you control become unexhausted.

#### Command

You may make as many actions during the Command phase as your resources allow. Those actions are:

- [Purchase](#purchasing-units) a unit
- [Move](#moving-units) a unit.
- Initiate a [battle](#battles).

#### Resupply

Any units which are not connected to your [logistics network](#logistic-network) are considered unsupplied. Unsupplied
Artillery, Convoy, and Infantry units will be destroyed, unsupplied Recon units suffer no consequences.
