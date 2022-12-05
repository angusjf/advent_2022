import Data.List
import Data.Sequence hiding (null, splitAt, filter, zip)
import Data.Foldable
import Data.Char

stacks = map (filter isLetter . snd) . filter (\(i, _) -> i `mod` 4 == 0) . zip [-1..] . transpose . init

cmd ["move", n, "from", a, "to", b] = (read n, read a - 1, read b - 1)

two stacks (n, a, b) = adjust (move ++) b $ update a keep stacks
    where (move, keep) = splitAt n $ stacks `index` a

solve (start, _:end) = foldl two (fromList $ stacks start) (map (cmd . words) end)

main = interact $ map head . toList . solve . break null . lines
