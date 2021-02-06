## How to use PackageCompiler

If you only want to execute the julia file then do `julia ./src/juliawer.jl`. If you want to compile the file with `PackageCompiler`
then you can follow the steps bellow:

1. Type `julia`
2. Type `]` and make sure you got `pkg>`.
3. Generate new project by `generate juliawer`
4. Press backspace and `Ctrl+D` in order to exit the `julia` command.
5. `cd juliawer`
6. Type `julia` (you could also ignore 4 and 5 and simply do `cd("juliawer")` before exiting the `julia` command.
7. Type: 
    ```julia
    using PackageCompiler  # assuming you have already installed it
    
    create_app(".", "compiled")  # this will take some time
    ```
8. Exit the `julia` command.
9. Go to the initial project dir `rust-wer` (one directory up from the newly created `Project.toml`).
10. Run `./juliawer/compiled/bin/juliawer data/mytranscripts.txt data/truth.txt`.


