// Every directive and argument form the lexer styles.
module example.com/app

go 1.24

toolchain go1.24.2

godebug (
	panicnil=1
)

require (
	example.com/lib/v2 v2.1.0-beta.1+incompatible // indirect
	golang.org/x/exp v0.0.0-20240506185415-9bf2ced13842
)

replace example.com/a v1.0.0 => example.com/b v1.1.0

replace example.com/c => "../local dir//c"

exclude example.com/bad v1.3.0

retract [v1.0.0, v1.1.0] // broken releases

tool example.com/cmd/x

ignore ./node_modules
