---
title: "Team Topologies"
date: 2022-12-09T13:43:35+01:00
draft: true
categories: ["books"]
tags: ["devops", "organization", "team", "book"]
---

Team Topologies: is an excellent guide to structuring and keeping teams, processes and technologies aligned, for all kinds of organisations. It gives us a vocabulary and a model to direct our thinking in a way that allows for scaling up, and helps us to organise teams for effective software development.

<!--more-->

Authors: Matthew Skelton & Manual Pais

## Preface

> Teams are always works in progress, but they are also your best shot at delivering value continuously and sustainably by aligning them with the business. 

> Long lived, autonomous, engaged members

## Chapter 1: The problem with Org Charts

> Businesses can no longer choose between optimizing for stability and optimizing for speed

> ... many organizations are still organizing people and teams in ways that are counterproductive to modern software development and operations

BVSSH: Previous ways of working were optimized for repetetive labor. It is nnecessary to optimize for early and often learning in a real environment with real customers or consumers. Work has moved from hand-making the same thing repeatedly (deterministic, known-unknowns) to **unique knowledge-based work** that is **emergent** and **full of unknown-unknowns**.

[![Cynefin (kun-nev-in)](https://upload.wikimedia.org/wikipedia/commons/thumb/f/f7/Cynefin_framework_by_Edwin_Stoop.jpg/1920px-Cynefin_framework_by_Edwin_Stoop.jpg)](https://upload.wikimedia.org/wikipedia/commons/thumb/f/f7/Cynefin_framework_by_Edwin_Stoop.jpg/1920px-Cynefin_framework_by_Edwin_Stoop.jpg)

Beter Value Sooner Safer Happier by Jonathan Smart

> ... fail to create the necessary conditions to embrace innovations while still delivering at a fast pace. In order to succeed at that, organisations need stable teems and effective team patterns and interactions.

> ... to treating people and technology as a single human/computer carbon/silicon sociotechnical ecosystem

> ... we need to ensure that teams are intrinsically motivated

Read "Drive: The Surprising Truth about What Motivates Us - Daniel Pink!"

### Communication Structures of an Organization

> The problem with taking the org chart ... end up trying to architect people as if they were software ... keeping their communication within the accepted lines.

### Org Chart Thinking Is the Problem

> Team Topologies focuses on how to set up dynamic team structures and interaction modes that can help teams adapt quickly to new conditions, and achieve fast and safe software delivery.

BVSSH: focus on outcomes. Agile, Lean, DevOps, design thinking, systems thinking, Theory of Constraints, and so on are all proverbial tools in a toolbox that organizations can employ to achieve desired outcomes. They are bodies of knowledge, years of wisdom acquired in the field of organized human endeavor, articulated as principles and practices. There is no cookie-cutter, one-size-fits-all approach.

> Eventually, you will face the issue of rigid team structures with poor communication and.or inadequate processes, slowing down delivery.

Again, BVSSH: old structures, industrial revolution type of leadership. TODO, add reference

> Thinking of the org chart as a faithful representation of how work gets done and how teams interact with each other leads to ineffective decisions around allocation of work and responsibilities.

Leadership, ownership: extreme ownership? 
- Example 1: chain of command: the periscope operator. Submarines normally dive. They don't use their periscope a lot and thus do not train a lot. So, to prevent mistakes, they introduced a supervisor. What happened: the operator became sloppy, thinking the supervisor will prevent him from making mistakes. The supervisor in turn, thinks the submarine commander will be supervising him so he is also not fully taking responsibility.
- More examples: read Turn the ship around: many examples of how having a chain of command leads to dependent people who do not take ownership.
- Example 3: Extreme Ownership: when people know the goal and have responsibility, they take ownership of their part, take autonomous decissions. 

### Beyond the Org Chart

> Three different organizational stuctures in every organisation:
> * Formal structure (the org chart) - facilitates compliance
> * Informal structure - the "realm of influence" between individuals
> * Value creation structure - how work actually gets done based on inter-personal and inter-team reputation

> ... the key to succesful knowledge work orgaizations is in the interactions between the informal structure and the value creation structure (Plaeging)

> Five rules of thumb for designing organizations:
> 1. Design when there is a compelling reason
> 2. Develop options for deciding on a design
> 3. Choose the right time to design
> 4. Look for clues that things are out of alignment
> 5. Stay alert to the future

Important information when starting with implementing Team Topologies!

### Team Topologies: A New Way of Thinking about Teams

> Team Topologies privides four fundamental team types: stream-aligned, platform, enabeling and complicated subsystem and three core team interaction modes - collaboration, X-as-a-Service and facilitating

> Conway's law, team cognitive load, how to become a sensing organization

### Cognitive Load and Bottlenecks

> Cognitive load, one person has a limit on how much information they can hold in their brains at any given moment

Code that fit's in your head: avg 7 things. [The Magical Number Seven, Plus or Minus Two](https://en.wikipedia.org/wiki/The_Magical_Number_Seven,_Plus_or_Minus_Two).

> ... we hardly ever discuss cognitive load when assigning responsibilities or software parts to a given team

How can we measure cognitive load, is it measurable?

> Team Topologies results in an effective and humanistic approach to building and running software systems.

> Dip in motivation: three elements of intrinsic motivation: 
> - autonomy - quashed by constant juggling of requests and priorities from multiple teams
> - mastery - jack of all trades, master of none
> - purpose - too many domains of responsibility

Difficult prioritizations and context switching, even in one sprint, a dip in people's motivation.

> ... development of new services is often planned as if the team had full-time availability and zero cognitive load to start with. 

Ownership and leadership again! An autonomous team can not blame others when this happens.

> Obstacles to fast flow:
> - Pushing against Conway's law
> - Software too big for teams
> - Disengaged teams
> - Too many suprises
> - Flow is blocked
> - Painful re-org every few years
> - Team pulled in many directions
> - Confusing org desing options

### Summary: Rethink Team Structures, Purpose, and Interactions

> The Agile, Lean IT, and DevOps movements helped demonstrate the enormous value of smaller, more autonomous teams that were aligned to the flow of business, developing and releasing in small, iterative cycles, and course correcting based on feedback from users.

> The **goal of Team Topologies** is to give you **the approach and mental tools** to **enable your organization to adapt and dynamically find** the places and timing when **collaboration** is needed, as well as when it is best to **focus on execution and reduce communication** overhead.

## Chapter 2: Conway's Law and Why It Matters

### Understanding Conway's Law

> If the architecture of the system and the architecture of the organization are at odds, the architecture of the organisation wins -  Ruth Malan

> Communication paths within an organisation effectively restrict the kinds of solutions that the organization can device. ... we can use this to our strategic advantage ... discourage certain kinds of design ... we can reshape the organization

> Organization design using Coinway's law becomes a key strategic activity that can greatly accelerate the discovery of effective software designs and help avoid those less effective.

> ... to increase an organization's chances of building effectice software sustems optimized for flow, a reverse Conway maneuver ... to reconfigure the team intercommunications before the software is finished. ... with sufficient willpower from **management** and **awareness from teams*** this approuch can and does work.

> _Accelerate_: The goal is for your architecture to support the ability of teams to get their work done - from designing trhough to deployment - without requiring high-bandwith communication between teams.

Story: during an interview I got a question about the size of teams. When I looked in the Slack channels, I saw that almost everyone is a member of all teams. They want to be kept in the loop. Difficult discussion because ofcourse information is important. But it was clearly a symptom of a larger problem: overcommunicating, everyone wanted to know everything **because** their work was intertwined, their were too much **dependencies** between teams.

### Software Architectures that Encourage Team-Scoped Flow

> ... we need to understand what software architecture is needed _before_ we organize our teams, otherwise the communication paths and incentives in the organization will end up dictating the software architecture.

> For a safe, rapid flow of changes, we need to consider team-scoped flow and design the software architecture to fit it. The fundamental means of delivery is the team, so the system architecture needs to enable and encourage fast flow within each team.

> ... follow proven software-architecture good practices:
> * Loose coupling
> * High cohesion
> * Clear and appropriate version compability
> * Clear and appropriate cross-team testing

> Architecture becomes an enabler, not a hindrance, but only if we take a team-first approach

### Organization Design Requires Technical Expertise

> anyone who makes decisions about the shape and placement of engineering teams is strongly influencing the software systems architecture. 

> _Ruth Malan:_ if we have managers deciding ... which services will be built, by which teams, we implicitly have managers deciding on the system architecture.

> ... it is very ineffective (perhaps irresponsible) for organizations that build software to decide on the shape, responsibilities, and boundaries of teams without input from technical leaders.

> _Allan Kelly:_ More than ever I believe that someone who claims to be an Architect needs both technical and social skills, they need to understand people and work within the social framework. They also need a remit that is broader than pure technology - they need to have a say in organizational structures and personnel issues, i.e. they need to be a manager too.

### Restrict Unnecessary Communication

> ... not all communication is good

> ... it is important to define "team interfaces" to set expectations around what kind of work requires strong collaboration and what doesn't.

> What we need is _focussed_ communication between specific teams. We need to look for unexpected communication and address the cause.

> A simple way to restrict communication is to move two teams to different parts of the office, different floors, or even different buildings. 

> for virtual teams or teams communicating over chat messengers, the volume and patterns of the team-to-team communications can help identify communications that do not match the interactions expected for the software architecture.

### Everyone Does Not Need to Communicate with Everyone

> If the organization has an expectation that "everyone should see every message in the chat" or "everyone need s to attend the massive standup meeting" or "everyone needs to be present in meetings" to approve decisions, then we have an organization design problem.

### Repeated Reorganizations that Create Fiefdoms or Reduce Headcount

> The underlying aim of many "reorganizations" in the past was to reduce staff or create fiefdoms of power for managersd and leaders.

> ... regular reorganizations for the sake of management convenience or reducing headcount actively destroy the ability of organizations to build and operate software systems effectively. 

### Summary: Conway's Law Is Critical for Effecient Team Design in Tech

> Fast flow requires restricting communication between teams. Team collaboration is important for gray areas of development, where discovery and expertise is needed to make progress. But in areas where execution prevails - not discovery - communication becomes an unnecessary overhead.

This kind of touches the Cynefin framework again!

## Chapter 3: Team-First Thinking

## Chapter 4: Static Team Topologies

## Chapter 5: The Four Fundamental; Team Topologies

## Chapter 6: Choose Team-First Boundaries

## Chapter 7: Team Interaction Modes

## Chapter 8: Evolve Team Structures with Organizational Sensing

## Conclusion: The Next-Generation Digital Operating Model
