import Data.List
import Data.Foldable
import Data.Char

main = interact $ solve . map parse . concatMap (take 2) . splitEvery 3 . lines

two = Pair (Pair (Cell 2) Nil) Nil

six = Pair (Pair (Cell 6) Nil) Nil

solve :: [Nest] -> String
solve xs =
    let
        s = zip [1..] $ sort $ [two, six] ++ xs
        [(x, _)] = filter (\(_, x) -> x == two) s
        [(y, _)] =  filter (\(_, x) -> x == six) s
    in
        show $ x * y

splitEvery n xs =
    case splitAt n xs of
        (start, []) -> [start]
        (start, end) -> start : splitEvery n end

pair [x, y] = (x, y)

data Nest = Cell Int | Pair Nest Nest | Nil deriving (Show, Eq)

parse :: String -> Nest
parse str =
    case runP str of
        Just (n, "") -> n
        Just (n, x) -> error x
        Nothing -> error "nothing"

numbers = map show [1..10]

runP :: String -> Maybe (Nest, String)
runP str =
    case str of
        '1':'0':more -> Just $ (Cell 10, more)
        '0':more -> Just $ (Cell 0, more)
        '1':more -> Just $ (Cell 1, more)
        '2':more -> Just $ (Cell 2, more)
        '3':more -> Just $ (Cell 3, more)
        '4':more -> Just $ (Cell 4, more)
        '5':more -> Just $ (Cell 5, more)
        '6':more -> Just $ (Cell 6, more)
        '7':more -> Just $ (Cell 7, more)
        '8':more -> Just $ (Cell 8, more)
        '9':more -> Just $ (Cell 9, more)
        '[':']':more ->
            Just (Nil, more)
        '[':more ->
            parseListItems more

parseListItems str =
    case runP str of
        Just (x, ']':more) ->
            Just $ (Pair x Nil, more)
        Just (x, ',':more) ->
            case parseListItems more of
                Just (xs, moremore) ->
                    Just $ (Pair x xs, moremore)

instance Ord Nest where
    compare = cmp

cmp :: Nest -> Nest -> Ordering
cmp (Cell a) (Cell b) = compare a b
cmp Nil Nil = EQ
cmp _ Nil = GT
cmp Nil _ = LT
cmp (Cell a) b = cmp (Pair (Cell a) Nil) b
cmp a (Cell b) = cmp a (Pair (Cell b) Nil)
cmp (Pair a as) (Pair b bs) =
    case cmp a b of
        LT -> LT
        GT -> GT
        EQ -> cmp as bs

-- cmp :: Nest -> Nest -> [(Nest, Nest, Bool)]

-- cmp (Cell a) (Cell b) = [(Cell a, Cell b, a <= b)]

-- cmp (Pair a as) (Pair b bs) = (cmp a b) ++ cmp as bs

-- cmp Nil Nil = [(Nil, Nil, True)]

-- cmp x Nil = [(x, Nil, False)]

-- cmp Nil x = [(Nil, x, True)]

-- cmp (Cell a) b = cmp (Pair (Cell a) Nil) b

-- cmp a (Cell b) = cmp a (Pair (Cell b) Nil)

