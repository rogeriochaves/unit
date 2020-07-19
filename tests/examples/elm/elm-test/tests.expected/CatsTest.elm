module CatsTest exposing (suite)

import Cats
import Expect
import Test exposing (..)


suite : Test
suite =
    test "it works" <|
        \_ ->
            (1 + 1)
                |> Expect.equal 2
