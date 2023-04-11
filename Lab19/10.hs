data Btree a = Emptree | Node a (Btree a) (Btree a) deriving (Show, Read, Eq, Ord)


maximumTree :: (Ord a) => Btree a -> a
maximumTree (Node a Emptree Emptree) = a
maximumTree (Node a l Emptree) = if a > lMax
                                    then a
                                    else lMax
    where lMax = maximumTree l
maximumTree (Node a Emptree r) = if a > rMax
									then a
									else rMax
    where rMax = maximumTree r
maximumTree (Node a l r) = if a > lMax && a > rMax
							then a
							else if lMax > rMax
							then lMax
							else rMax
    where lMax = maximumTree l
          rMax = maximumTree r