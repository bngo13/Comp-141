data Dog = Dog String String Int deriving (Show, Eq)

breed (Dog _ b _) = b