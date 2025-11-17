from invoke import Collection, task

import setup
from utils.command_executor import CommandExecutor
from utils.windows.environment_utils import activate_VS2022_environment
from utils.logger import print_task_documentation


@task(pre=[setup.setup_context])
@print_task_documentation
def check(ctx):
    """
    Check leet_code rust-project
    """

    command = ["cargo check"]

    CommandExecutor(ctx).execute(command, cwd=f"{ctx.leet_code_dir}", log="leet_code-check.log")


@task(pre=[setup.setup_context])
@print_task_documentation
def build(ctx):
    """
    Build leet_code rust-project
    """

    command = [
        activate_VS2022_environment(),
        ["cargo build"],
    ]

    CommandExecutor(ctx).execute(command, cwd=f"{ctx.leet_code_dir}", log="leet_code-build.log")


collection = Collection("leet_code")
collection.add_task(check, name="check")
collection.add_task(build, name="build")
