maximum' :: [Int] -> Int
maximum' = maximum'Helper 0

maximum'Helper :: Int -> [Int] -> Int
maximum'Helper acc [] = acc
maximum'Helper acc (x:xs) = maximum'Helper (max x acc) xs