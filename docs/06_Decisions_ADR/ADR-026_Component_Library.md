# ADR-010 - Component Library als Single Source of Truth

## Status
Beschlossen

## Entscheidung
PraxisQM verwendet eine zentrale Component Library. Jede UI-Komponente besitzt eine eindeutige COMP-ID und wird genau einmal beschrieben. Andere Dokumente referenzieren Komponenten nur noch ueber ihre ID.

## Begruendung
So werden Redundanzen und widerspruechliche Beschreibungen vermieden. Aenderungen an Komponenten erfolgen zentral.
