var searchIndex = JSON.parse('{\
"annealers":{"doc":"","i":[[0,"model","annealers","",null,null],[3,"SingleModel","annealers::model","",null,null],[3,"FixedSingleQuadricModel","","",null,null],[3,"Prods","","",null,null],[3,"Neighbors","","",null,null],[8,"ModelView","","",null,null],[16,"Node","","",0,null],[16,"NodesIter","","",0,null],[10,"node","","",0,[[]]],[10,"nodes","","",0,[[]]],[11,"size","","",0,[[]]],[8,"SingleModelView","","",null,null],[16,"Node","","",1,null],[16,"NodesIter","","",1,null],[16,"ProdsIter","","",1,null],[16,"NeighborsIter","","",1,null],[16,"Order","","",1,null],[10,"order","","",1,[[]]],[10,"node","","",1,[[]]],[10,"nodes","","",1,[[]]],[11,"size","","",1,[[]]],[11,"get_weight","","",1,[[]]],[10,"get_weight_unchecked","","Safetyp is in nodes()",1,[[]]],[10,"prods","","",1,[[]]],[10,"neighbors","","",1,[[]]],[11,"calculate_prod","","",1,[[["singlesolution",3]]]],[8,"FixedSingleModelView","","",null,null],[16,"Node","","",2,null],[16,"Order","","",2,null],[10,"order","","",2,[[]]],[10,"node","","",2,[[]]],[10,"size","","",2,[[]]],[11,"get_weight","","",2,[[]]],[10,"get_weight_unchecked","","SafetyAll items of p are less than size()",2,[[]]],[11,"new","","",3,[[]]],[11,"add_weight","","",3,[[]]],[11,"new","","",4,[[]]],[11,"add_weight","","",4,[[]]],[11,"new","","",5,[[]]],[11,"new","","",6,[[]]],[0,"node","annealers","",null,null],[3,"DiscreteNode","annealers::node","",null,null],[3,"Spin","","",null,null],[3,"Binary","","",null,null],[3,"TwoVal","","",null,null],[8,"Node","","",null,null],[16,"RealType","","",7,null],[8,"SingleNode","","",null,null],[16,"RealType","","",8,null],[10,"get_value","","",8,[[]]],[11,"calculate_prod","","",8,[[]]],[11,"new","","",9,[[]]],[11,"new","","",10,[[]]],[11,"new","","",11,[[]]],[0,"repr","annealers","",null,null],[3,"BinaryRepr","annealers::repr","",null,null],[3,"BinaryReprIter","","",null,null],[11,"new_random","","",12,[[]]],[11,"with_len_unchecked","","SafetyGiven len is less than len()",12,[[]]],[11,"from_vec","","",12,[[]]],[11,"to_vec","","",12,[[],["vec",3]]],[11,"len","","",12,[[]]],[11,"is_empty","","",12,[[]]],[11,"get","","",12,[[]]],[11,"set","","",12,[[]]],[11,"set_unchecked","","SafetyGiven loc is less than len()",12,[[]]],[11,"get_unchecked","","SafetyGiven loc is less than len()",12,[[]]],[11,"flip","","",12,[[]]],[11,"flip_unchecked","","SafetyGiven loc is less than len()",12,[[]]],[11,"iter","","",12,[[],["binaryrepriter",3]]],[0,"set","annealers","",null,null],[8,"NodeSet","annealers::set","",null,null],[16,"Iter","","",13,null],[11,"into_set","","",13,[[],["btreeset",3]]],[11,"into_vec","","",13,[[],["vec",3]]],[11,"from_set","","",13,[[["btreeset",3]],["option",4]]],[11,"from_vec","","",13,[[["vec",3]],["option",4]]],[11,"from_vec_unchecked","","SafetyGiven vec must be sorted.",13,[[["vec",3]],["option",4]]],[10,"from_it","","",13,[[["intoiterator",8]],["option",4]]],[10,"iter","","",13,[[]]],[11,"len","","Important`len()` should return non-zero value.",13,[[]]],[11,"contains","","",13,[[]]],[0,"solution","annealers","",null,null],[3,"SingleSolution","annealers::solution","",null,null],[12,"state","","",14,null],[12,"energy","","",14,null],[12,"occurrences","","",14,null],[12,"local_field","","",14,null],[8,"Solution","","",null,null],[16,"Node","","",15,null],[11,"from_vec","","Generate SingleSolution from qubit values",14,[[]]],[11,"from_state","","Generate SingleSolution from BinaryRepr",14,[[["binaryrepr",3]]]],[11,"len","","Get the number of solutions",14,[[]]],[11,"compare_energy","","Compare two SingleSolution by energy.",14,[[],[["ordering",4],["option",4]]]],[11,"with_local_field","","Ensure that SingleSolution has local field.",14,[[]]],[11,"with_energy","","Ensure that SingleSolution has energy.",14,[[]]],[11,"calculate_energy","","Calculate energy with model.",14,[[]]],[11,"calculate_local_field","","Calculate local field with model.",14,[[],["vec",3]]],[11,"get","","Get the qubit value located in `index`.",14,[[]]],[11,"get_unchecked","","Get the qubit value located in `index`.",14,[[]]],[0,"solver","annealers","Solver module contains abstraction of many solver type.",null,null],[3,"AsStructuredSolverGeneratorWrapper","annealers::solver","",null,null],[3,"UnstructuredEdgeIter","","",null,null],[8,"SolverGenerator","","",null,null],[16,"SolverType","","",16,null],[16,"ErrorType","","",16,null],[11,"value_range","","",16,[[]]],[10,"generate","","",16,[[],["result",4]]],[8,"StructuredSolverGenerator","","",null,null],[10,"nodes","","",17,[[],[["box",3],["iterator",8]]]],[10,"prods","","",17,[[],[["box",3],["iterator",8]]]],[8,"UnstructuredSolverGenerator","","",null,null],[16,"Order","","",18,null],[10,"order","","",18,[[]]],[11,"size","","",18,[[],["option",4]]],[11,"into_structured","","",18,[[],["asstructuredsolvergeneratorwrapper",3]]],[8,"Solver","","",null,null],[16,"ErrorType","","",19,null],[16,"SolutionType","","",19,null],[8,"ClassicalSolver","","",null,null],[10,"solve_with_rng","","",20,[[],[["result",4],["vec",3]]]],[8,"AsyncSolver","","",null,null],[10,"solve_async","","",21,[[],[["box",3],["pin",3]]]],[8,"SyncSolver","","",null,null],[10,"solve","","",22,[[],[["result",4],["vec",3]]]],[0,"variable","annealers","",null,null],[8,"Real","annealers::variable","",null,null],[18,"MAX","","",23,null],[18,"MIN","","",23,null],[10,"as_f64","","",23,[[]]],[10,"from_i32","","",23,[[]]],[10,"from_f64","","",23,[[]]],[10,"abs","","",23,[[]]],[10,"min","","",23,[[]]],[10,"max","","",23,[[]]],[10,"nan_or","","",23,[[]]],[10,"is_finite","","",23,[[]]],[11,"zero","","",23,[[]]],[11,"one","","",23,[[]]],[8,"ConvertForce","","This trait is implemented between all Real types.",null,null],[10,"convert_force","","",24,[[]]],[8,"ConvertFrom","","",null,null],[10,"convert_from","","",25,[[]]],[8,"ConvertTo","","",null,null],[10,"convert_to","","",26,[[]]],[8,"ConvertWith","","",null,null],[16,"Output","","",27,null],[10,"convert_lhs","","",27,[[]]],[10,"convert_rhs","","",27,[[]]],[0,"prelude","annealers","",null,null],[0,"order","","",null,null],[3,"Quadric","annealers::order","",null,null],[3,"HighOrder","","",null,null],[8,"Order","","",null,null],[16,"NodeSetType","","",28,null],[10,"order","","",28,[[]]],[11,"new","","",29,[[]]],[11,"node","annealers::model","",3,[[]]],[11,"nodes","","",3,[[]]],[11,"size","","",3,[[]]],[11,"order","","",3,[[]]],[11,"node","","",3,[[]]],[11,"nodes","","",3,[[]]],[11,"prods","","",3,[[]]],[11,"neighbors","","",3,[[]]],[11,"get_weight","","",3,[[]]],[11,"get_weight_unchecked","","",3,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"node","","",4,[[]]],[11,"nodes","","",4,[[]]],[11,"size","","",4,[[]]],[11,"order","","",4,[[]]],[11,"node","","",4,[[]]],[11,"nodes","","",4,[[]]],[11,"prods","","",4,[[]]],[11,"neighbors","","",4,[[]]],[11,"get_weight","","",4,[[]]],[11,"get_weight_unchecked","","",4,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"into_iter","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"into_iter","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"from","annealers::node","",30,[[]]],[11,"into","","",30,[[]]],[11,"to_owned","","",30,[[]]],[11,"clone_into","","",30,[[]]],[11,"borrow","","",30,[[]]],[11,"borrow_mut","","",30,[[]]],[11,"try_from","","",30,[[],["result",4]]],[11,"try_into","","",30,[[],["result",4]]],[11,"type_id","","",30,[[],["typeid",3]]],[11,"vzip","","",30,[[]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"vzip","","",11,[[]]],[11,"from","annealers::repr","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"vzip","","",12,[[]]],[11,"from","","",31,[[]]],[11,"into","","",31,[[]]],[11,"into_iter","","",31,[[]]],[11,"borrow","","",31,[[]]],[11,"borrow_mut","","",31,[[]]],[11,"try_from","","",31,[[],["result",4]]],[11,"try_into","","",31,[[],["result",4]]],[11,"type_id","","",31,[[],["typeid",3]]],[11,"vzip","","",31,[[]]],[11,"from","annealers::solution","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"vzip","","",14,[[]]],[11,"from","annealers::solver","",32,[[]]],[11,"into","","",32,[[]]],[11,"borrow","","",32,[[]]],[11,"borrow_mut","","",32,[[]]],[11,"try_from","","",32,[[],["result",4]]],[11,"try_into","","",32,[[],["result",4]]],[11,"type_id","","",32,[[],["typeid",3]]],[11,"vzip","","",32,[[]]],[11,"from","","",33,[[]]],[11,"into","","",33,[[]]],[11,"into_iter","","",33,[[]]],[11,"borrow","","",33,[[]]],[11,"borrow_mut","","",33,[[]]],[11,"try_from","","",33,[[],["result",4]]],[11,"try_into","","",33,[[],["result",4]]],[11,"type_id","","",33,[[],["typeid",3]]],[11,"vzip","","",33,[[]]],[11,"from","annealers::order","",34,[[]]],[11,"into","","",34,[[]]],[11,"to_owned","","",34,[[]]],[11,"clone_into","","",34,[[]]],[11,"borrow","","",34,[[]]],[11,"borrow_mut","","",34,[[]]],[11,"try_from","","",34,[[],["result",4]]],[11,"try_into","","",34,[[],["result",4]]],[11,"type_id","","",34,[[],["typeid",3]]],[11,"vzip","","",34,[[]]],[11,"from","","",29,[[]]],[11,"into","","",29,[[]]],[11,"to_owned","","",29,[[]]],[11,"clone_into","","",29,[[]]],[11,"borrow","","",29,[[]]],[11,"borrow_mut","","",29,[[]]],[11,"try_from","","",29,[[],["result",4]]],[11,"try_into","","",29,[[],["result",4]]],[11,"type_id","","",29,[[],["typeid",3]]],[11,"vzip","","",29,[[]]],[11,"order","annealers::model","",3,[[]]],[11,"node","","",3,[[]]],[11,"nodes","","",3,[[]]],[11,"get_weight","","",3,[[]]],[11,"get_weight_unchecked","","Safetyit is always safe",3,[[]]],[11,"prods","","",3,[[]]],[11,"neighbors","","",3,[[]]],[11,"node","","",4,[[]]],[11,"order","","",4,[[]]],[11,"size","","",4,[[]]],[11,"get_weight_unchecked","","",4,[[]]],[11,"get_value","annealers::node","",9,[[]]],[11,"get_value","","",10,[[]]],[11,"get_value","","",11,[[]]],[11,"generate","annealers::solver","",32,[[],["result",4]]],[11,"nodes","","",32,[[],[["box",3],["iterator",8]]]],[11,"prods","","",32,[[],[["box",3],["iterator",8]]]],[11,"order","annealers::order","",34,[[]]],[11,"order","","",29,[[]]],[11,"next","annealers::model","",5,[[],["option",4]]],[11,"next","","",6,[[],["option",4]]],[11,"next","annealers::repr","",31,[[],["option",4]]],[11,"next","annealers::solver","",33,[[],[["btreeset",3],["option",4]]]],[11,"clone","annealers::model","",3,[[],["singlemodel",3]]],[11,"clone","","",4,[[],["fixedsinglequadricmodel",3]]],[11,"clone","annealers::node","",30,[[],["discretenode",3]]],[11,"clone","","",9,[[],["spin",3]]],[11,"clone","","",10,[[],["binary",3]]],[11,"clone","","",11,[[],["twoval",3]]],[11,"clone","annealers::repr","",12,[[],["binaryrepr",3]]],[11,"clone","annealers::solution","",14,[[],["singlesolution",3]]],[11,"clone","annealers::order","",34,[[],["quadric",3]]],[11,"clone","","",29,[[],["highorder",3]]],[11,"default","annealers::node","",9,[[]]],[11,"default","","",10,[[]]],[11,"cmp","annealers::order","",34,[[["quadric",3]],["ordering",4]]],[11,"cmp","","",29,[[["highorder",3]],["ordering",4]]],[11,"eq","","",34,[[["quadric",3]]]],[11,"eq","","",29,[[["highorder",3]]]],[11,"ne","","",29,[[["highorder",3]]]],[11,"partial_cmp","","",34,[[["quadric",3]],[["ordering",4],["option",4]]]],[11,"partial_cmp","","",29,[[["highorder",3]],[["ordering",4],["option",4]]]],[11,"lt","","",29,[[["highorder",3]]]],[11,"le","","",29,[[["highorder",3]]]],[11,"gt","","",29,[[["highorder",3]]]],[11,"ge","","",29,[[["highorder",3]]]],[11,"fmt","annealers::repr","",12,[[["formatter",3]],["result",6]]],[11,"fmt","annealers::order","",34,[[["formatter",3]],["result",6]]],[11,"fmt","","",29,[[["formatter",3]],["result",6]]],[11,"index","annealers::repr","",12,[[]]],[11,"index","annealers::solution","",14,[[]]],[11,"hash","annealers::order","",34,[[]]],[11,"hash","","",29,[[]]]],"p":[[8,"ModelView"],[8,"SingleModelView"],[8,"FixedSingleModelView"],[3,"SingleModel"],[3,"FixedSingleQuadricModel"],[3,"Prods"],[3,"Neighbors"],[8,"Node"],[8,"SingleNode"],[3,"Spin"],[3,"Binary"],[3,"TwoVal"],[3,"BinaryRepr"],[8,"NodeSet"],[3,"SingleSolution"],[8,"Solution"],[8,"SolverGenerator"],[8,"StructuredSolverGenerator"],[8,"UnstructuredSolverGenerator"],[8,"Solver"],[8,"ClassicalSolver"],[8,"AsyncSolver"],[8,"SyncSolver"],[8,"Real"],[8,"ConvertForce"],[8,"ConvertFrom"],[8,"ConvertTo"],[8,"ConvertWith"],[8,"Order"],[3,"HighOrder"],[3,"DiscreteNode"],[3,"BinaryReprIter"],[3,"AsStructuredSolverGeneratorWrapper"],[3,"UnstructuredEdgeIter"],[3,"Quadric"]]},\
"classical_solver":{"doc":"Default annealing implementation.","i":[[4,"NoneError","classical_solver","`NoneError` means the error will never be returned. It…",null,null],[0,"algo","","",null,null],[5,"simulated_annealing","classical_solver::algo","",null,[[["binaryrepr",3]]]],[0,"beta","classical_solver","",null,null],[4,"BetaType","classical_solver::beta","",null,null],[13,"Count","","Specify beta schedule by beta count.",0,null],[13,"CountRange","","Specify beta schedule by beta count and beta range.",0,null],[13,"Schedule","","Specify beta schedule manually. This values should take…",0,null],[0,"sa","classical_solver","",null,null],[3,"SimulatedAnnealerGenerator","classical_solver::sa","",null,null],[12,"sweeps_per_round","","",1,null],[12,"beta","","",1,null],[3,"SimulatedAnnealer","","",null,null],[11,"new","","",1,[[]]],[11,"from","classical_solver","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_string","","",2,[[],["string",3]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"from","classical_solver::beta","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","classical_solver::sa","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"clone","classical_solver::beta","",0,[[],["betatype",4]]],[11,"clone","classical_solver::sa","",1,[[],["simulatedannealergenerator",3]]],[11,"fmt","classical_solver::beta","",0,[[["formatter",3]],["result",6]]],[11,"fmt","classical_solver::sa","",1,[[["formatter",3]],["result",6]]],[11,"fmt","classical_solver","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"generate","classical_solver::sa","",1,[[],["result",4]]],[11,"order","","",1,[[],["quadric",3]]],[11,"solve_with_rng","","",3,[[],[["noneerror",4],["result",4],["vec",3]]]]],"p":[[4,"BetaType"],[3,"SimulatedAnnealerGenerator"],[4,"NoneError"],[3,"SimulatedAnnealer"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);