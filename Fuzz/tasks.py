from invoke import task, Collection

import setup
import leet_code
from utils.logger import INFO
from utils.command_executor import CommandExecutor


@task(pre=[setup.setup_context])
def get_info(ctx):
    """
    Print to console information about active configuration of invoke-tasks
    """
    env_context = setup.get_env_context()

    # Define custom formatters for string columns
    formatters = {}
    for col in env_context.select_dtypes(include='object').columns:
        max_len = env_context[col].astype(str).str.len().max()
        formatters[col] = lambda x, length=max_len: f"  {x:<{length}s}"

    INFO.log_line("Active environment configuration:") \
        .log_line(f"{env_context.to_string(formatters=formatters, index=False)}")


@task(pre=[setup.setup_context])
def yapf(ctx):
    """
    Format python files in Fuzz
    """
    command = [
        "yapf",
        "--style .style.yapf",
        "--verbose",
        "--in-place",
        "--recursive ./leet_code ./utils",
        "tasks.py",
    ]

    CommandExecutor(ctx).execute(command, cwd=f"{ctx.fuzz_dir}", log="yapf.log")


namespace = Collection()
namespace.add_task(get_info, name="get-info")
namespace.add_task(yapf, name="yapf")
namespace.add_collection(leet_code.collection)
