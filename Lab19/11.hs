data Btree a = Emptree | Node a (Btree a) (Btree a) deriving (Show, Read, Eq, Ord)

isBST :: (Ord a) => Btree a -> Bool
isBST Emptree = True
isBST (Node val Emptree Emptree) = True
isBST (Node val (Node leftVal Emptree Emptree) (Emptree)) = leftVal < val
isBST (Node val (Emptree) (Node rightVal Emptree Emptree)) = val > rightVal
isBST (Node val (Node left lleft lright) (Node right rleft rright)) = (isBST lleft) && isBST lright && isBST rleft && (isBST rright) && val > left && val < right