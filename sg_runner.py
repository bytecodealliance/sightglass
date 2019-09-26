import os
import datetime
import time
from datetime import datetime
import subprocess
import argparse
import json
import collections
from collections import OrderedDict
import fileinput
import sys
import textwrap

# Changed in Docker
# And outside of Docker
parser = argparse.ArgumentParser(
    formatter_class=argparse.RawDescriptionHelpFormatter,
    epilog=textwrap.dedent('''
            available runtimes:     base_native - LLVM-8
                                    lucet_app - https://github.com/fastly/lucet (latest via git pull)
                                    wasmtime_app - https://github.com/CraneStation/wasmtime (latest via git pull)
                                    node - https://github.com/nodejs/node (latest via nvm)

            available perf-suites:  shootout

            example usage: ./sg_container_runner.sh -p shootout -r wasmtime_app
         '''))

parser.add_argument('-r','--runtime-list', nargs='+', help='<Required> List of runtimes to use for performance runs', required=True)
parser.add_argument('-p','--perf-suites', nargs='+', help='<Required> List of performance suites to run', required=True)
parser.add_argument('-s','--send', help="Upload to corresponding sightglass UI, results for each performance suite. Corresponding UI location defined in config.inc.",
                    action="store_true")
parser.add_argument('--no-default-base_native', help="Do not run base_native implementation unless specified in --runtime-list arguments. Note baseline is alwasy the first in the alphabetized list of runtimes.",
                    action="store_true")
parser.add_argument('--skip-downloads', help="Do not fetch the latest sources. Only of use if binaries from previous builds exist.",
                    action="store_true")


#### Configuration ####

## Global Variables
## TODO: Read from config.inc. Will likely remove.
CWD = os.getcwd();
SOURCE = "source"
args = parser.parse_args()
ARGS_DICT = vars(args)
PERF_TEST = False;
DATE_TIME = datetime.now().strftime('%Y-%m-%d')

## Dictionaries
## TODO: Scan predefined directory to populate
perf_suites = {
    "shootout" : "./benchmark/shootout/",
    #"polybench" : "./benchmark/polybench/",
}

## TODO: Read from config.inc to populate
send_url = {
    "shootout" : "localhost:8001/submit",
    "polybench" : "localhost:8002/submit",
}

# Appended by build_dics()
download = collections.OrderedDict()
shootout = collections.OrderedDict()
polybench = collections.OrderedDict()
sha = collections.OrderedDict()

# Build dictionaries based on cmd flags and directory file structure
def build_dics():
    cwd = os.getcwd();
    for vm in ARGS_DICT['runtime_list']:
        vm_download_path = cwd + "/plugs/" + vm + "/download.sh";
        if not os.path.exists(vm_download_path) and vm != "base_native":
            print ("\nCannot find download script {}.\nWill ignore all requests for {}.\n".format(vm_download_path, vm));
            continue
        valid_vm = False;
        for suite in ARGS_DICT['perf_suites']:
            suite_build_path = cwd + "/plugs/" + vm + "/" + suite + "/build.sh";
            if os.path.exists(suite_build_path):
                valid_vm = True;
                if not args.no_default_base_native and not 'base_native' in eval(suite):
                    base_native_build_path = cwd + "/plugs/base_native/" + suite + "/build.sh";
                    if os.path.exists(base_native_build_path):
                        print ("Adding base_native to {} perf list.".format(suite));
                        eval(suite)['base_native'] = base_native_build_path;
                print ("Adding {} to {} perf list.".format(vm, suite));
                eval(suite)[vm] = suite_build_path;
            else:
                print ("\nCannot find {}.\nWill not add {} for performance runs of {}.\n".format(suite_build_path, vm, suite));
        if valid_vm:
            global PERF_TEST;
            PERF_TEST = True;
            if vm != "base_native" and not args.skip_downloads:
                if not 'wasi_libc' in download:
                    print ("Adding wasi_libc to vm download list.");
                    download['wasi_libc'] = os.path.join(CWD, "plugs/wasi_libc/download.sh");
                print ("Adding {} to vm download list.".format(vm));
                download[vm] = vm_download_path;


# Send data to SG history UI(s)
def send_data(suite, json_data_file):

    time_stamp=int(time.time());
    if args.no_default_base_native:
        baseline = ARGS_DICT['runtime_list'][0]
    else:
        baseline = "base_native"

    for vm in ARGS_DICT['runtime_list']:

        # Open data file and remove what's not needed
        dest_json_data_file = "{}/{}-{}-{}".format(os.path.dirname(json_data_file), vm, suite, os.path.basename(json_data_file));
        #print ("Creating data file: {}".format(dest_json_data_file));
        with open(dest_json_data_file, 'w') as target:
            with open(json_data_file, 'r') as source:
                json_data = json.load(source)
                for json_workload in json_data:
                    for json_vm in json_workload[1]:
                        #print ("\nVM: " + vm + " is type: {}".format(type(vm)))
                        #print ("Baseline: " + baseline + " is type: {}".format(type(baseline)))
                        if not ((vm == json_vm[0]) or (baseline == json_vm[0])):
                            json_workload[1].remove(json_vm)
                            break;
                target.write(json.dumps(json_data))
                sys.stdout.flush()

        time.sleep(1)
        submission_id=int(time.time());
        source=SOURCE;

        ## Call method for message
        echo_message_1 = CWD + "/plugs/" + vm + "/" + suite + "echo_build.sh"
        echo_message_2 = CWD + "/plugs/" + vm + "/" + "echo_build.sh"
        if os.path.exists(echo_message_1):
            message= subprocess.check_output(echo_message_1, shell=True).strip()
        elif os.path.exists(echo_message_2):
            message= subprocess.check_output(echo_message_2, shell=True).strip()
        else:
            message="gitref:TODO";
        vm=vm;
        data=dest_json_data_file;
        send_cmd="./results/send.sh {} {} {} {} {} {} {}".format(send_url[suite], submission_id, \
                                                                    source, message, vm, time_stamp, data);
        os.system(send_cmd);


# Driver main loop
def main():

    # Build dictionaries
    build_dics()

    if not PERF_TEST:
        print ("No valid requests. Exiting ...");
        return;

    # Download all
    # TODO .. build wasi_libc first
    for vm in download:
        print ("Executing download script: {}".format(download[vm]));
        os.chdir(os.path.dirname(download[vm]));
        sha_return = subprocess.check_output(download[vm], shell=False, stderr = subprocess.PIPE)
        os.chdir(CWD);


    # Build workloads
    for suite in perf_suites:
        for vm in eval(suite):
            print ("Executing build script: {}".format(eval(suite)[vm]));
            sha_return = subprocess.check_output(eval(suite)[vm], shell=False, stderr = subprocess.PIPE)
 

    # Build sightglass
    build_return = subprocess.check_output(["cargo", "build", "--manifest-path=sightglass/Cargo.toml", "--release"], shell=False, stderr = subprocess.PIPE);

    # Create and Run sightglass toml for each workload
    for suite in perf_suites:
        valid_toml = False;
        toml_name = suite + "-latest.toml";
        os.popen("cp sg_template.toml {}".format(toml_name));
        sys.stdout.flush()
        time.sleep(1)
        for vm in eval(suite):
            valid_toml = True;
            implementation=CWD+"/plugs/" + vm + "/" + suite + "/bin/implementation.so"
            if os.path.exists(implementation):
                toml_line = "{ name = \"" + vm + "\", library_path = \"" + implementation + "\" },"
                print ("Inserting {} in {}".format(vm, toml_name));
                for line in fileinput.input(toml_name, inplace=True):
                    line = line.replace("#DO_NOT_REMOVE", "{}\n#DO_NOT_REMOVE".format(toml_line));
                    sys.stdout.write(line)
                    sys.stdout.flush()
            else:
                print ("Cannot find {}\nWill not do perf runs for {}.\n".format(implementation, suite));
                valid_toml = False;
                break;

        if valid_toml:
            os.system("./sightglass/target/release/sightglass -c {}".format(toml_name));

            # Archive and send results if required
            archive_dir = CWD+"/results/"+suite;
            results_json=CWD+"/results/results-latest.json";
            results_csv=CWD+"/results/results-latest.csv";
            if not os.path.exists(archive_dir):
                os.makedirs(archive_dir)

            os.popen("cp {} {}/".format(results_json, archive_dir));
            os.popen("cp {} {}/".format(results_csv, archive_dir));
            os.popen("cp {} {}/{}-{}.json".format(results_json, archive_dir, suite, DATE_TIME));
            os.popen("cp {} {}/{}-{}.csv".format(results_csv, archive_dir, suite, DATE_TIME));

            if args.send:
                send_data(suite, results_json)

            print ("Perf test for {} done.".format(suite));
            print ("");
            print ("See: {}/results/{}.html.".format(CWD, suite));
            print ("Ex:");
            print ("    python3 -m http.server 7800 &");
            print ("    firefox http://localhost:7800/results/{}.html".format(suite));
            print ("or:");
            print ("    google-chrome --allow-file-access-from-files results/{}.html".format(suite));
    print ("")

if __name__ == '__main__':
    main()
