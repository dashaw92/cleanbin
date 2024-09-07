module ProjectTypes
open System
open System.Diagnostics

type ProjectType = { searchFor: string -> bool; command: string; args: string list }

let cleanProject projType path =
    let psi = new ProcessStartInfo (projType.command, projType.args)
    psi.UseShellExecute <- false
    psi.RedirectStandardOutput <- false
    psi.RedirectStandardError <- false
    psi.CreateNoWindow <- true
    psi.WorkingDirectory <- path
    
    printfn $"Cleaning %s{path}"
    let exec = Diagnostics.Process.Start psi
    exec.WaitForExit ()

module Registry =
    let KnownTypes: ProjectType list =
        [
            { searchFor = ((=) "Cargo.toml"); command = "cargo"; args = [ "clean" ] }
            { searchFor = (fun path -> path.EndsWith ".fsproj"); command = "dotnet"; args = [ "clean" ] }
            { searchFor = (fun path -> path.EndsWith ".csproj"); command = "dotnet"; args = [ "clean" ] }
        ]
