open System
open System.IO
open System.Diagnostics

open ProjectTypes

let getProjectType file =
    Registry.KnownTypes
    |> List.tryFind (fun proj -> proj.searchFor(file))

let getProject (path: string) =
    getProjectType (Path.GetFileName path)
    |> Option.map (fun projType -> projType, Path.GetDirectoryName path)

let getProjects basePath =
    Directory.EnumerateFiles (basePath, "*", SearchOption.AllDirectories)
    |> Seq.map getProject

let runWith path =
    path
    |> getProjects
    |> Seq.filter Option.isSome
    |> Seq.map Option.get
    |> Seq.iter ((<||) cleanProject)

    0

[<EntryPoint>]
let main = function
    | [| path |] -> runWith path
    | _ -> 
        eprintfn "Expected only a base path to search for projects."
        1
