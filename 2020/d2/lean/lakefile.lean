import Lake
open Lake DSL

package «aoc» where
  -- add package configuration options here

require Parser from git "https://github.com/fgdorais/lean4-parser" @ "main"

@[default_target]
lean_exe «aoc» where
  root := `Main
  -- Enables the use of the Lean interpreter by the executable (e.g.,
  -- `runFrontend`) at the expense of increased binary size on Linux.
  -- Remove this line if you do not need such functionality.
  supportInterpreter := true
