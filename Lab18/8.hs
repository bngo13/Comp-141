mapTR :: (a -> b) -> [a] -> [b]
mapTR = mapTRHelper []

mapTRHelper :: [b] -> (a -> b) -> [a] -> [b]
mapTRHelper acc funct [] = reverse acc
mapTRHelper acc funct (x:xs) = mapTRHelper (funct x:acc) funct xs