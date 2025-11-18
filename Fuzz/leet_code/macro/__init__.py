from invoke import Collection, task

import setup
from utils.command_executor import CommandExecutor
from utils.windows.environment_utils import activate_VS2022_environment
from utils.logger import print_task_documentation


@task(pre=[setup.setup_context])
@print_task_documentation
def check(ctx):
    """
    Check leet_code_macro rust-lib
    """

    command = ["cargo check"]

    CommandExecutor(ctx).execute(command, cwd=f"{ctx.leet_code_macro_dir}", log="leet_code_macro-check.log")


@task(pre=[setup.setup_context])
@print_task_documentation
def build(ctx):
    """
    Build leet_code_macro rust-lib
    """

    command = [
        activate_VS2022_environment(),
        ["cargo build"],
    ]

    CommandExecutor(ctx).execute(command, cwd=f"{ctx.leet_code_macro_dir}", log="leet_code_macro-build.log")


@task(pre=[setup.setup_context])
@print_task_documentation
def test(ctx):
    """
    Test leet_code_macro rust-lib
    """

    command = [
        activate_VS2022_environment(),
        ["cargo test"],
    ]

    CommandExecutor(ctx).execute(command, cwd=f"{ctx.leet_code_macro_dir}", log="leet_code_macro-test.log")


collection = Collection("macro")
collection.add_task(check, name="check")
collection.add_task(build, name="build")
collection.add_task(test, name="test")
