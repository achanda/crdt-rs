var searchIndex = {};
searchIndex['crdt_rs'] = {"items":[[3,"GSet","crdt_rs","A `GSet` is an implementation of a grow-only set.\nThe underlying data-structure is a `BTreeSet`",null,null],[3,"PSet","","Implements a state based two-phase set\nusing two `GSet`s",null,null],[11,"fmt","","",0,{"inputs":[{"name":"gset"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",0,null],[11,"clone","","",0,{"inputs":[{"name":"gset"}],"output":{"name":"gset"}}],[11,"new","","Creates a new, empty `GSet`",0,{"inputs":[{"name":"gset"}],"output":{"name":"gset"}}],[11,"insert","","Inserts an element of type `T` into the\ngiven `GSet`. Returns `true` if the element\nwas already in the set. Otherwise, it inserts\nthe element and returns `false`",0,{"inputs":[{"name":"gset"},{"name":"t"}],"output":{"name":"bool"}}],[11,"contains","","Checks if the given value of type `T` is in\nthe set. If the check is successful, returns\n`true`. Otherwise returns `false`.",0,{"inputs":[{"name":"gset"},{"name":"t"}],"output":{"name":"bool"}}],[11,"len","","Returns the number of elements in the given `GSet`",0,{"inputs":[{"name":"gset"}],"output":{"name":"usize"}}],[11,"is_empty","","Checks if the given `GSet` is empty",0,{"inputs":[{"name":"gset"}],"output":{"name":"bool"}}],[11,"difference","","Returns the set difference between two `GSet`s as another `GSet`",0,{"inputs":[{"name":"gset"},{"name":"gset"}],"output":{"name":"gset"}}],[11,"intersection","","Returns the intersection between two `GSet`s as another `GSet`",0,{"inputs":[{"name":"gset"},{"name":"gset"}],"output":{"name":"gset"}}],[11,"union","","Returns the union between two `GSet`s as another `GSet`",0,{"inputs":[{"name":"gset"},{"name":"gset"}],"output":{"name":"gset"}}],[11,"from_iter","","",0,{"inputs":[{"name":"gset"},{"name":"i"}],"output":{"name":"gset"}}],[11,"extend","","",0,{"inputs":[{"name":"gset"},{"name":"iter"}],"output":null}],[11,"into_iter","","",0,{"inputs":[{"name":"gset"}],"output":{"name":"intoiter"}}],[11,"fmt","","",1,{"inputs":[{"name":"pset"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",1,null],[11,"clone","","",1,{"inputs":[{"name":"pset"}],"output":{"name":"pset"}}],[11,"new","","Creates a new `PSet`",1,{"inputs":[{"name":"pset"}],"output":{"name":"pset"}}],[11,"insert","","Inserts an element of type `T` into the\ngiven `PSet`. Returns `true` if the element\nwas already in the set. Otherwise, it inserts\nthe element and returns `false`",1,{"inputs":[{"name":"pset"},{"name":"t"}],"output":{"name":"bool"}}],[11,"remove","","Removes the given element from the `PSet`",1,{"inputs":[{"name":"pset"},{"name":"t"}],"output":{"name":"bool"}}],[11,"contains","","Checks if the given value of type `T` is in\nthe set. If the check is successful, returns\n`true`. Otherwise returns `false`",1,{"inputs":[{"name":"pset"},{"name":"t"}],"output":{"name":"bool"}}],[11,"contents","","Returns the contents of the given set. This is\nequivalent to the set difference between the add set and the\nremove set",1,{"inputs":[{"name":"pset"}],"output":{"name":"vec"}}],[11,"is_empty","","Checks if the given `PSet` is empty",1,{"inputs":[{"name":"pset"}],"output":{"name":"bool"}}],[11,"len","","Returns the length of the given `PSet`",1,{"inputs":[{"name":"pset"}],"output":{"name":"usize"}}],[11,"union","","Returns the set union between the given `PSet` and\nanother `PSet` as a `PSet`",1,{"inputs":[{"name":"pset"},{"name":"pset"}],"output":{"name":"pset"}}],[11,"intersection","","Returns the set intersection between the given `PSet` and\nanother `PSet` as a `PSet`",1,{"inputs":[{"name":"pset"},{"name":"pset"}],"output":{"name":"pset"}}],[11,"difference","","Returns the set difference between the given `PSet` and\nanother `PSet` as a `PSet`",1,{"inputs":[{"name":"pset"},{"name":"pset"}],"output":{"name":"pset"}}],[11,"from_iter","","",1,{"inputs":[{"name":"pset"},{"name":"i"}],"output":{"name":"pset"}}],[11,"extend","","",1,{"inputs":[{"name":"pset"},{"name":"iter"}],"output":null}]],"paths":[[3,"GSet"],[3,"PSet"]]};
initSearch(searchIndex);