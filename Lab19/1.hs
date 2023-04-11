data Day = Monday | Tuesday | Wednesday | Thursday | Friday | Saturday | Sunday deriving (Eq)

workday input
	| input == Monday = True
	| input == Tuesday = True
	| input == Wednesday = True
	| input == Thursday = True
	| input == Friday = True
	| otherwise = False