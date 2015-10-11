module Sieve where

import Data.DList ( DList, empty, toList, snoc )

data Sieve = Sieve (DList Int) [Maybe Int] deriving (Eq,Show)

numbersUpTo :: Int -> [Maybe Int]
numbersUpTo m
  | m >= 2 = map Just [2..m]
  | otherwise = []

discriminate :: Sieve -> Sieve
discriminate (Sieve primes nbs) = case nbs of
  (Nothing:xs) -> discriminate $ Sieve primes xs
  (Just p:_) -> discriminate $ Sieve (primes `snoc` p) (tail $ killEach p nbs)
  _ -> Sieve primes []

sieve :: Int -> Sieve
sieve m = Sieve empty (numbersUpTo m)

-- TODO: non-tail recursive, might destroy stack!
killEach :: (Ord a) => Int -> [Maybe a] -> [Maybe a]
killEach m = go 1
  where
    go _ [] = []
    go q (x:xs)
      | q <= 1 = Nothing : go m xs
      | otherwise = x : go (pred q) xs

prime :: Int -> [Int]
prime n = let Sieve primes _ = discriminate (sieve n) in toList primes
