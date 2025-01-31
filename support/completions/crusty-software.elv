
use builtin;
use str;

set edit:completion:arg-completer[crusty-software] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'crusty-software'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'crusty-software'= {
            cand -c 'Config location file or directory'
            cand --config 'Config location file or directory'
            cand -v 'Verbose output'
            cand --verbose 'Verbose output'
            cand -d 'Dry run'
            cand --dry-run 'Dry run'
            cand -k 'Keep running if a step failed'
            cand --keep 'Keep running if a step failed'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
