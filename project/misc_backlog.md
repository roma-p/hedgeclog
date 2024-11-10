---
id: misc_backlog
aliases: []
tags: []
---

Proto: un niveau une idée de gameaplay (pas forcément aboutis jusqu'au fond)
ici le but c'est plutôt d'arriver à faire tourner 30 systèmes différents qui 

Avant de commencer: au moins 15 idées de mécaniques pour les hérissons.

* Editor feature: 
  - [ ] Auto add/removal of all walls that are not exit (faster edition...)
  - [ ] Auto solver...
  - [ ] tracing; tracing path (used tiles) and unused tiles (by anyone) to remove fluff more easily...

* Maxi TODOLIST:
  - [ ] Use less strict code: ensure query worked, otherwise return without panick...
  - [x] Camera shall contains all its logic so:
   - [x] instantiate its camera info (except the level position?, to see with level hot reload)
   - [x] snap to all different position handled by the camera...
  - [x] use event on left key...
  - [x] split editor code into sub files...
  - [x] change visibility of existing tile undor cursor
  - [x] decalage entre souris et tile... tile id false?
  - [x] bug sur la suppression des tiles existantes...
  - [x] Rework editor
   - [x] split "main"
   - [x] how to load in parrallel? https://www.phind.com/search?cache=eg70im8j93bxrzzz6d4x367t
   - [x] event system to only change cube when needed in selector view.
   - [x] debug current implementation of gameview.
   - [x] rename gameview to levelview
   - [x] event system for gameview
   - [x] rename sub system... -> tile_select / tile_add_remove
   - [x] crash occasionel need a setup state: "NotLoaded / Loading / Loaded_Not_Setup / Loaded_And_Setup
   - [x] change ViewState: ViewTileSelection / ViewLevel.
   - [x] add rotation
   - [x] plein de crash: due a la gestion des events? devrait pluôt avoir un state entre les deux?add rotation
   - [x] PROBABLEMENT TOUT RÉÉCRIRE... EN FINISSANT DE REGARDER LES VIDÉOS...
    - [x] Keep messages for user input. but with correct flush point.
    - [x] loading / dispose and such: no message...
   - [x] setup / loading in a "setup_dispose" subsystem
   - [x] hide selector cube by default, reveal it when enterring normal mode.
   - [x] esc when selecting nodes does not work....
   - [x] envoyer event "actualize grid position" when entering tile mode!
   - [x] OnEnter / OnExit tile : visibility.
   - [ ] move all single / single_mut -> get_single / get_single_mut (no panicking... but good for debug...)
   - [ ] snap camera done using event, a ressource with the position to snap too and send an event.
   - [ ] if create anything but a floor, delete evantual hedgclog.
   - [ ] add a clean logic? remove hedgclog when removing groudn or creating antthing but a floor?
   - [ ] clean all warnings...
   - [ ] maybe single "hover" logic -> in "cursor to world position" => gives you current hovering tile / hedgeclog once for all.
   - [ ] Centralize input dectection system to rpevent spam...
   - [ ] !!!! STORE  TILES / HEDEGHOG ENTITIES in the RGrid. Will Simplify heavily the code.
   - [ ] Bug: if I create a tile, not move the hovering tile, remove it, it is not removed...
   - [ ] ET DONC grand survey pr supprimer tous les "for all entities"... simplifiera pas mal les queries! (tous les "update tile creator position etc...)
   - [ ] Conform level (will work for objects and everythin)
    - [ ] 1. create / remove tile / hedgehog logic in core module, not in add remove in editor.
    - [ ] 2. new plugin? conform tile register on 'LevelTileModified'
    - [ ] 3. query all levels to just check that this is correct? not great but...
     - [ ] if no tile: no hedgehog
     - [ ] if tile is anything but floor: no hedgehog.
     - [ ] double remove marche pas, add multiple tiles on same position seems to work (sometime remove does not work)
   - [ ] validate logic when leaving editor (and saving level...)
   - [ ] save edit / discard edit. -> handle multiple levels at once basics...
   - [ ] tsunami animation to transition between too level.
   - [ ] MAINTENANT: être capable de spawn / despawn un niveau...
    - [ ] 1. "Level Description component"
    - [ ] 2. Arriver à la serialiser qqpart.
    - [ ] 3. build la ressource "gridlevel" depuis un level description.
    - [ ] 4. faire de la ressource un component pr gérer le multi level loading.
   - [ ] Only create hedeghog if cursor on tile.


** arc1 Bricolage dans tous les sens avant de réflechir à comment stocker / loader les niveaux
   - [x] snap camera done using event, a ressource with the position to snap too and send an event.
   - [x] déplacer snap de camera.
   - [ ] add hedgclog
   - [ ] menu objet?
   - [ ] flame sur les cases de feu -> sprite dont la position "hérite" de la case fille.
   - [x] suppression de tiles.
   - [ ] trouver pourquoi ca lag qd on rentre en mode tile... on dirait qu'un truc est loadé au lieu d'être setup...
   - [ ] documenter qui est setup / load...
   - [ ] UI changement de mode hegheog à fix.
   - [ ] bug changement de mode: tile hoover not shown.

  Bundle tile can't have optionnal... so either a method to instantiate every tile.
  Either a separate method to "prepare them" on instanciation. (but what about movement?)
  Either a struct telling who's who. Eihter marker components for each tile.
  In the second solution, I need a component saying "prepared True/False".

  Utiliser les ev uniquement pour les fx... même pas pour le scheduling vraiment...
  En réécrivant l'éditor: un év "current action" "nothing" / "waiting" et tous les autres.
  Waiting: time.sleep(). ne schedule userinput que tu "nothing".
  Trigger les ev sur "on transition : "nothing" -> "action'. (enfin tester si faire next(mystate) from mystate trigger qqchose...
  De manière générale, les inputs là, c'est pas contrôlé. Ajouter ca...
