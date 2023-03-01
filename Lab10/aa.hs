findInd :: (Eq a) => a -> [a] -> Int
findInd _ [] = error "Not in list"
findInd find (x:xs) = if find == x then 0 else 0 + findInd find (xs)

toUppr :: Char -> Char
toUppr inChar = if inChar `elem` ['a'..'z'] then ['A'..'Z'] !! findInd inChar ['a'..'z'] else inChar