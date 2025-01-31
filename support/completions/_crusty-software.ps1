
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'crusty-software' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'crusty-software'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'crusty-software' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Config location file or directory')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Config location file or directory')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Verbose output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Dry run')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Dry run')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'Keep running if a step failed')
            [CompletionResult]::new('--keep', '--keep', [CompletionResultType]::ParameterName, 'Keep running if a step failed')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
