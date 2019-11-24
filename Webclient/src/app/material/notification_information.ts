import { Severity } from "../domain_value/severity";

export class NotificationInformation {
  constructor(public readonly severity: Severity,
              public readonly message: string
    ) {}
}
