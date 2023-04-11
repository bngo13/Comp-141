tailScuff :: [a] -> Maybe [a]

tailScuff [] = Nothing
tailScuff (x:xs) = Just xs