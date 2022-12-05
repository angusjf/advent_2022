module Main where

import Data.List
import Data.Sequence hiding (null, splitAt, filter, reverse)
import Data.Foldable
import Data.Char

chunks n = map toList . toList . chunksOf n . fromList

stacks = map (concatMap (filter isLetter)) . transpose . init . map (chunks 4)
      
cmd ["move", n, "from", a, "to", b] = (read n, read a - 1, read b - 1)

two stacks (n, a, b) = adjust (move ++) b $ update a keep stacks
    where (move, keep) = splitAt n $ stacks `index` a

main = interact $ \input ->
    let (start, _:end) = break null $ lines input
    in map head $ toList $ foldl two (fromList $ stacks start) (map (cmd . words) end)