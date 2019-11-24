import {Severity} from "../domain_value/severity";

export class Notification {
  constructor(public readonly severity: Severity,
              public readonly message: string
  ) {
  }
}
