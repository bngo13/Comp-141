data Btree a = Emptree | Node a (Btree a) (Btree a) deriving (Show, Read, Eq, Ord)

heightTree :: Btree a -> Int
heightTree Emptree = -1
heightTree (Node _ left right) = if 1  + (heightTree left) > 1  + (heightTree right) then 1  + (heightTree left) else 1  + (heightTree right)

asfd = (
			Node 3
			(
				Node 2
				(
					Emptree
				)
				(
					Emptree
				)
			)
			(
				Node 4
				(
					Node 2
					Emptree
					Emptree
				)
				(
					Node 3
					Emptree
					Emptree
				)
			)
		)