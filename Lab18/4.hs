sum' :: (Num a) => a -> [a] -> a
sum' acc [] = acc
sum' acc (x:xs) = sum' (acc + x) xs