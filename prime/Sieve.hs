module Sieve where

prime :: Int -> [Int]
prime n = sieve [2..n]
  where
    sieve (p:xs) = p : sieve (filter (\x -> x `rem` p /= 0) xs)
    sieve _ = []
