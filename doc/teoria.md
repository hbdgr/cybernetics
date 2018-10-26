// Nie tłumaczyć! no ale już za późno.

// axiomatic theory of cognition

// Eight axioms
// 1. any elementary object 'o_a' is identicat with itself 'o_a'
// 2. any elementary object 'o_a' is not_identical with any relation 'r_ab'
// 3. any relation 'r_ab' is identical with itself 'r_ab'
// 4. any relation 'r_ab' in not_identical with any elementary object 'o_a'
// 5. any elementary object 'o_a' (where 'a' can be any of 1,2,...,n) is part_of set of elementary objects 'O'
// 6. any elementary object 'o_a' is not_part_of set of relations 'R'
// 7. any relation 'r_ab' (where 'a','b' can be any of 1,2,...,n) is part_of set of relations 'R'
// 8. any relation 'r_ab' is not_part_of set of elementary objects 'O'

// Five secondary definitions
// 1. characteristic := Being a part_of set (determine the sets that make up the 'concept' of)
// 2. disorder := lack of identity between
// 3. arrangement := introdution of relations to the elementary objects set
// 4. system := (O_x, R_x) ≡ 'S_x' - arrangement of sets of objects and relations
// 5. substance 'S_x' :=  objects sets that are part_of system 'S_x'
// 6. structure 'R_x' := relations sets that are part_of system 'S_x'

////////////////////////////////////////////////////////////////////////////////////////////////

Nadsystem - zbiór systemów oraz relacji między nimi.
Definicja - określenie przynależności do zbioru.


/********************************* objects and relations concept **********************************

// elementary object 'o'
// 'o_i' - i-th elementary object
pub struct ElementaryObject {
	content: String,
}

// relations 'r'
// r_ab - (a,b means id of elementary objects between which the relationship occurs)
pub struct Relation {
	a: ElementaryObject,
	b: ElementaryObject,
	description: String,
}

// collection
// 'O' - elementary objects set
// 'R' - relations set
pub struct ObjectsSet {
	objects: Vec<ElementaryObject>,
}

pub struct RelationsSet {
	relations: Vec<Relation>,
}

// System: 'S_x' ≡ (O_x, R_x)
pub struct System {
	substance: ObjectsSet,
	structure: RelationsSet,
	name: String,
}

pub struct SystemRelation {
	a: System,
	b: System,
	description: String,
}

// Oversystem, contains systems, and relations between them
pub struct Oversystem {
	systems: Vec<System>,
	over_relations: Vec<SystemRelation>
}


enum PrimaryRelations {
	part_of,			// ∈
	not_part_of,		// ∉
	identical, 			// ≡
	not_identical,	// !≡
}

enum FlowRelations {
	subscribe,
}

*/// ----------------------------------------------------------------------------------------------
