param (
    [Parameter(Mandatory=$true)]
    [int]$N
)

for ($i = 1; $i -le $N; $i++) {
    & cargo run -- add "todo $i"
}
