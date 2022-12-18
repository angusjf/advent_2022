import qualified Data.Map as M
import Debug.Trace
import Data.List

stripSuffix :: String -> String -> String
stripSuffix suffix xs =
    if drop n xs == suffix then take n xs else xs
  where n = length xs - length suffix

parse :: String -> (String, (Int, [String]))
parse line =
    let _:valve:_:_:rate:_:_:_:_:leadsTo = words line
    in (valve, (read $ stripPrefix_ "rate=" $ init $ rate, map (stripSuffix ",") leadsTo))

stripPrefix_ pre xs =
    case stripPrefix pre xs of
        Just x -> x
        _ -> error "stripPrefix_"

lookup_ k map =
    case M.lookup k map of
        Just x -> x
        _ -> error "lookup_"

main :: IO ()
main =
    do
        testData <- readFile "src/bin/test16-1.txt"
        putStrLn (show $ one $ M.fromList $ map parse $ lines testData)

type Map = M.Map String (Int, [String])

one :: Map -> [String]
one valves =
    path 5 [] "AA" valves


-- -- -- -- -- -- -- -- -- -- -- -- -- 


path :: Int -> [String] -> String -> Map -> [String]
path mins opened cave valves =
    if mins <= 0 then
        []
    else
        let
            (next_cave, new_opened) =
                let
                    (_, links_to) = lookup_ cave valves
                    choices =
                        (cave, value_of_opening_valve_at_n_mins_and_after (mins - 1) opened valves cave)
                        :
                        (map (\dest -> (dest, value_of_being_in_cave_at_n_mins (mins - 1) opened valves dest)) links_to)
                    
                    best_choice = 
                        trace ("<" ++ (show choices) ++ ">") $ last $ map fst $ sortOn snd $ choices
                in
                    if best_choice == cave then
                        (cave, cave : opened)
                    else
                        (best_choice, opened)
        in
            cave : path (mins - 1) new_opened next_cave valves


value_of_opening_valve_at_n_mins_and_after :: Int -> [String] -> Map -> String -> Int
value_of_opening_valve_at_n_mins_and_after 0 _ _ _ = 0
value_of_opening_valve_at_n_mins_and_after mins opened valves cave =
    let
        (r, links_to) = lookup_ cave valves
        value_of_next_step = last $ sort $ (map (value_of_being_in_cave_at_n_mins (mins - 1) (cave : opened) valves) links_to)
    in
        (r * mins) + value_of_next_step

value_of_being_in_cave_at_n_mins :: Int -> [String] -> Map -> String -> Int
value_of_being_in_cave_at_n_mins 0 _ _ _ = 0
value_of_being_in_cave_at_n_mins (- 1) _ _ _ = 0
value_of_being_in_cave_at_n_mins mins opened valves cave =
    let
        value_as_stepping_stone = 
            let
                (r, links_to) = lookup_ cave valves
            in
                last $ sort $ (map (value_of_being_in_cave_at_n_mins (mins - 1) opened valves) links_to)
    in
        if cave `elem` opened then
            -- can not be opened
            value_as_stepping_stone
        else
            -- compare value of opening vs value of not opening
            max (value_of_opening_valve_at_n_mins_and_after (mins - 1) opened valves cave) value_as_stepping_stone

