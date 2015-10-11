import Data.Bits ( (.&.), shiftL, shiftR )
import Data.Vector ( Vector, fromList )
import System.Environment ( getArgs )
import System.Exit ( ExitCode(ExitFailure), exitFailure, exitWith )

main :: IO ()
main = do
  args <- getArgs
  case args of
    [lstr,acgtChain] -> do
      let lparsed = reads lstr
      case lparsed of
        [(l,[])] -> solve l acgtChain
        _ -> exitWith (ExitFailure 2)
    _ -> exitFailure

solve :: Int -> String -> IO ()
solve l = go
  where
    go [] = pure ()
    go chain = do
      let window = take l chain
      putStrLn window
      go (tail chain)

allChains :: Int -> Int -> String -> [String]
allChains m l = go m
  where
    go _ [] = []
    go m' chain
      | m' >= l = take l chain : go (m' - 1) (tail chain)
      | otherwise = []

type Hash = Int

hash :: String -> Hash
hash = foldl (\a x -> shiftL a 2 + acgtHash x) 0

acgtHash :: Char -> Int
acgtHash c = case c of
  'a' -> 0 -- 00
  'c' -> 1 -- 01
  'g' -> 2 -- 10
  't' -> 3 -- 11
  _   -> error "acgtHash: unknown nucleobase"

unhash :: Int -> Hash -> String
unhash l h = go shiftJump
  where
    go 0   = [acgtUnhash h]
    go jmp = acgtUnhash (shiftR h jmp) : go (jmp - 2)
    shiftJump = l * 2 - 2

acgtUnhash :: Int -> Char
acgtUnhash i = case i .&. 3 of
  0 -> 'a'
  1 -> 'c'
  2 -> 'g'
  3 -> 't'
  i -> error $ "acgtUnhash: unknown hash: " ++ show i
