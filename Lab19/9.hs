data Btree a = Emptree | Node a (Btree a) (Btree a) deriving (Show, Read, Eq, Ord)

minimumTree :: (Ord a) => Btree a -> a
minimumTree (Node leaf (Node left Emptree Emptree) Emptree) = if leaf < left then leaf else left
minimumTree (Node leaf Emptree (Node right Emptree Emptree)) = if leaf < right then leaf else right
minimumTree (Node leaf Emptree Emptree) = leaf
minimumTree (Node leaf left right) = let leftVal = minimumTree left
                                         rightVal = minimumTree right
                                     in
                                         if leaf < leftVal && leaf < rightVal then leaf
                                                                              else if leftVal < leaf && leftVal < rightVal then leftVal
                                                                                                                           else rightVal