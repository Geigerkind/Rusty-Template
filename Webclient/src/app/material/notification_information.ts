import { Severity } from "../domainvalue/severity";

export class NotificationInformation {
  constructor(public readonly severity: Severity,
              public readonly message: string
    ) {}
}
