open System
open System.IO
open System.Diagnostics

let getProjects basePath =
    Directory.EnumerateFiles (basePath, "Cargo.toml", SearchOption.AllDirectories)
    |> Seq.map Path.GetDirectoryName

let cleanCmd path =
    let psi = new ProcessStartInfo("cargo", [ "clean" ])
    psi.UseShellExecute <- false
    psi.RedirectStandardOutput <- false
    psi.RedirectStandardError <- false
    psi.CreateNoWindow <- true
    psi.WorkingDirectory <- path
    
    printfn "Cleaning %s" path
    let exec = Diagnostics.Process.Start psi
    exec.WaitForExit ()

let runWith path =
    path
    |> getProjects
    |> Seq.iter cleanCmd

    0

[<EntryPoint>]
let main = function
    | [| path |] -> runWith path
    | _ -> 
        eprintfn "Expected only a base path to search for projects."
        1
