import { exec } from "child_process";
import semver from "semver";
import { VyperSettings } from "./types";

export class Compiler {
  constructor(private _pathToVyper: string) {}

  /**
   *
   * @param inputPaths array of paths to contracts to be compiled
   */
  public async compile(
    inputPaths: string[],
    compilerVersion: string = "",
    settings: VyperSettings = {}
  ) {
    const output: string = await new Promise((resolve, reject) => {
      const settingsCmd = getSettingsCmd(compilerVersion, settings);

      const process = exec(
        `${this._pathToVyper} ${settingsCmd} -f combined_json ${inputPaths.join(
          " "
        )}`,
        {
          maxBuffer: 1024 * 1024 * 500,
        },
        (err, stdout) => {
          if (err !== null) {
            return reject(err);
          }
          resolve(stdout);
        }
      );

      process.stdin!.end();
    });

    return JSON.parse(output);
  }
}

function getSettingsCmd(
  compilerVersion: string,
  settings: VyperSettings
): string {
  let settingsStr =
    settings.evmVersion !== undefined
      ? `--evm-version ${settings.evmVersion} `
      : "";

  settingsStr += getOptimize(compilerVersion, settings.optimize);

  return settingsStr;
}

function getOptimize(
  compilerVersion: string,
  optimize: string | boolean | undefined
): string {
  if (compilerVersion === "" && optimize !== undefined) {
    throw new Error(
      "The 'compilerVersion' parameter must be set when the setting 'optimize' is set."
    );
  }

  if (optimize === undefined) {
    return "";
  }

  if (typeof optimize === "boolean") {
    if (optimize) {
      if (
        semver.gte(compilerVersion, "0.3.10") ||
        semver.lt(compilerVersion, "0.3.1")
      ) {
        throw new Error(
          `The 'optimize' setting with value 'true' is not supported for versions of the Vyper compiler older than 0.3.1 or newer than 0.3.10. You are currently using version ${compilerVersion}.`
        );
      }

      // The optimizer is enabled by default
      return "";
    } else {
      return semver.lt(compilerVersion, "0.3.10")
        ? "--no-optimize"
        : "--optimize none";
    }
  }

  if (typeof optimize === "string") {
    if (semver.gte(compilerVersion, "0.3.10")) {
      return `--optimize ${optimize}`;
    }

    throw new Error(
      `The 'optimize' setting, when specified as a string value, is available only starting from the Vyper compiler version 0.3.10. You are currently using version ${compilerVersion}.`
    );
  }

  throw new Error(
    `The 'optimize' setting has an invalid type value. Type is: ${typeof optimize}.`
  );
}
