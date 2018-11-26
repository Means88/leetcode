import Data.List.Split

main = do
  line <- getLine
  let [n, k] = map (\x -> read x :: Double) (splitOn " " line)
  putStrLn $ show $ sum $ map (\x -> ceiling (x * n / k)) [2, 5, 8]

