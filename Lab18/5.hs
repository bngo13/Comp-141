cnct :: String -> [String] -> String
cnct acc [] = acc
cnct acc (x:xs) = cnct (acc ++ x) xs