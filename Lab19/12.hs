data Tree a = EmptyTree | NodeTree a [Tree a]

asdf = NodeTree 1 [NodeTree 5 [NodeTree 2 [EmptyTree]], NodeTree 8 [EmptyTree], NodeTree 7 [NodeTree 3 [EmptyTree], NodeTree 4 [EmptyTree]]]