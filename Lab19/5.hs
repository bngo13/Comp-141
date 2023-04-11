data Btree a = Emptree | Node a (Btree a) (Btree a) deriving (Show, Read, Eq, Ord)

tree = Node 4
	(
		Node 3
		(Emptree)
		(Emptree)
	)
	(
		Node 5
		(Emptree)
		(Emptree)
	)

multree :: Btree Double -> Double
multree (Node double Emptree Emptree) = double
multree (Node double left right) = double * (multree left) * (multree right)

singleton :: a -> Btree a
singleton x = Node x Emptree Emptree

treeInsert :: (Ord a) => a -> Btree a -> Btree a
treeInsert x Emptree = singleton x
treeInsert x (Node a left right)
	| x == a = Node x left right
	| x < a = Node a (treeInsert x left) right
	| x > a = Node a left (treeInsert x right)

makeBST :: (Ord a) => [a] -> Btree a
makeBST [x] = Node x (Emptree) (Emptree)
makeBST (x:xs) = treeInsert x (makeBST xs)