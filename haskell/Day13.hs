import Data.List
import Data.Char

main = interact $ show . pt2

pt2 :: String -> Int
pt2 = product .
      map fst .
      filter ((`elem` [two, six]) . snd) .
      zip [1..] .
      sort .
      (++) [two, six] .
      map (fst . parse) .
      concatMap (take 2) .
      splitEvery 3 .
      lines

data Nest = Cell Int | Pair Nest Nest | Nil deriving (Show, Eq)

two = Pair (Pair (Cell 2) Nil) Nil

six = Pair (Pair (Cell 6) Nil) Nil

parse str =
    case str of
        '1':'0':more -> (Cell 10, more)
        d:more | isDigit d -> (Cell (digitToInt d), more)
        '[':']':more ->
            (Nil, more)
        '[':more ->
            parseListItems more

parseListItems str =
    case parse str of
        (x, ']':more) -> (Pair x Nil, more)
        (x, ',':more) -> let (xs, moremore) = parseListItems more
                         in  (Pair x xs, moremore)

instance Ord Nest where
    compare (Cell a) (Cell b) = compare a b
    compare Nil Nil = EQ
    compare _ Nil = GT
    compare Nil _ = LT
    compare (Cell a) b = compare (Pair (Cell a) Nil) b
    compare a (Cell b) = compare a (Pair (Cell b) Nil)
    compare (Pair a as) (Pair b bs) =
        case compare a b of
            LT -> LT
            GT -> GT
            EQ -> compare as bs

splitEvery n xs =
    case splitAt n xs of
        (start, []) -> [start]
        (start, end) -> start : splitEvery n end

