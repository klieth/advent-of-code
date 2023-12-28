import Lake
open Lake DSL

package «aoc»

-- importing parser below implies importing std; we only need to import this if it isn't included another way
--require std from git "https://github.com/leanprover/std4" @ "v4.3.0"

require Parser from git "https://github.com/fgdorais/lean4-parser" @ "0dc1d2018b11bcfba2d7acbee95828526f795703"

@[default_target]
lean_exe «aoc» where
  root := `Main
  -- Enables the use of the Lean interpreter by the executable (e.g.,
  -- `runFrontend`) at the expense of increased binary size on Linux.
  -- Remove this line if you do not need such functionality.
  supportInterpreter := true

