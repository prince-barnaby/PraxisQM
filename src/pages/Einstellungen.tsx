import { useState } from "react";
import {
  Settings,
  Hash,
  FolderTree,
  UserCog,
  DatabaseBackup,
  Info,
} from "lucide-react";
import SettingsNav, {
  type SettingsNavItem,
} from "../components/settings/SettingsNav";
import SettingsSection from "../components/settings/SettingsSection";
import MasterDataSection from "../components/settings/MasterDataSection";
import {
  fetchResponsibilities,
  createResponsibility,
  renameResponsibility,
  fetchQmAreas,
  createQmArea,
  renameQmArea,
} from "../lib/masterDataApi";
import "./Einstellungen.css";

const SETTINGS_SECTIONS: SettingsNavItem[] = [
  { id: "allgemein", label: "Allgemein", icon: Settings },
  { id: "dokumentennummerierung", label: "Dokumentennummerierung", icon: Hash },
  { id: "kategorien", label: "Kategorien & Unterkategorien", icon: FolderTree },
  { id: "benutzerverwaltung", label: "Benutzerverwaltung", icon: UserCog },
  { id: "backup", label: "Backup", icon: DatabaseBackup },
  { id: "systeminformationen", label: "Systeminformationen", icon: Info },
];

export default function Einstellungen() {
  const [activeSection, setActiveSection] = useState("allgemein");

  return (
    <div className="pqm-einstellungen">
      <header className="pqm-einstellungen__header">
        <h1 className="pqm-einstellungen__title">Einstellungen</h1>
        <p className="pqm-einstellungen__subtitle">
          Konfiguration der PraxisQM-Anwendung
        </p>
      </header>

      <div className="pqm-einstellungen__layout">
        <div className="pqm-einstellungen__nav">
          <SettingsNav
            items={SETTINGS_SECTIONS}
            activeId={activeSection}
            onSelect={setActiveSection}
          />
        </div>

        <div className="pqm-einstellungen__content">
          {activeSection === "allgemein" && (
            <SettingsSection
              title="Allgemein"
              description="Übergreifende Anwendungseinstellungen"
            >
              <div className="pqm-settings-placeholder">
                In diesem Bereich werden zukünftig allgemeine
                Anwendungseinstellungen angezeigt. Die Konfiguration ist
                aktuell nicht implementiert.
              </div>
            </SettingsSection>
          )}

          {activeSection === "dokumentennummerierung" && (
            <SettingsSection
              title="Dokumentennummerierung"
              description="Automatische Vergabe und Verwaltung von Dokumentennummern"
            >
              <div className="pqm-settings-info">
                <div className="pqm-settings-info__row">
                  <span className="pqm-settings-info__label">
                    Nummerierungsschema
                  </span>
                  <span className="pqm-settings-info__value pqm-settings-info__value--mono">
                    PQM-#### (Platzhalter)
                  </span>
                </div>
                <div className="pqm-settings-info__row">
                  <span className="pqm-settings-info__label">
                    Automatische Vergabe
                  </span>
                  <span className="pqm-settings-info__value">
                    Ja — Dokumentennummern werden automatisch erzeugt (ADR-001)
                  </span>
                </div>
                <div className="pqm-settings-info__row">
                  <span className="pqm-settings-info__label">Unveränderlich</span>
                  <span className="pqm-settings-info__value">
                    Ja — Dokumentennummern sind unveränderlich (ADR-001)
                  </span>
                </div>
                <div className="pqm-settings-info__row">
                  <span className="pqm-settings-info__label">
                    Wiederverwendung
                  </span>
                  <span className="pqm-settings-info__value">
                    Nein — Dokumentennummern werden niemals wiederverwendet
                    (ADR-001)
                  </span>
                </div>
              </div>
              <div className="pqm-settings-placeholder pqm-settings-placeholder--warning">
                Dokumentennummern sind automatisch, unveränderlich und werden
                niemals wiederverwendet. Eine manuelle Bearbeitung bereits
                vergebener Nummern ist nicht möglich.
              </div>
            </SettingsSection>
          )}

          {activeSection === "kategorien" && (
            <SettingsSection
              title="Kategorien & Unterkategorien"
              description="Verwaltung von Verantwortungspositionen und QM-Bereichen"
            >
              <MasterDataSection
                title="Verantwortungspositionen"
                description="Zentral verwaltbare QM-Verantwortungen, die Mitarbeitenden zugewiesen werden können."
                inputLabel="Bezeichnung"
                addButtonLabel="Verantwortungsposition hinzufügen"
                emptyMessage="Noch keine Verantwortungspositionen angelegt."
                loadingMessage="Verantwortungspositionen werden geladen …"
                duplicateHint="Diese Bezeichnung existiert bereits."
                fetchItems={fetchResponsibilities}
                createItem={createResponsibility}
                renameItem={renameResponsibility}
              />
              <MasterDataSection
                title="QM-Bereiche"
                description="Zentral verwaltbare Qualitätsmanagementbereiche, die Mitarbeitenden zugewiesen werden können."
                inputLabel="Bezeichnung"
                addButtonLabel="QM-Bereich hinzufügen"
                emptyMessage="Noch keine QM-Bereiche angelegt."
                loadingMessage="QM-Bereiche werden geladen …"
                duplicateHint="Diese Bezeichnung existiert bereits."
                fetchItems={fetchQmAreas}
                createItem={createQmArea}
                renameItem={renameQmArea}
              />
            </SettingsSection>
          )}

          {activeSection === "benutzerverwaltung" && (
            <SettingsSection
              title="Benutzerverwaltung"
              description="Verwaltung von Benutzerkonten mit Login und Rollen"
            >
              <div className="pqm-settings-placeholder">
                In diesem Bereich werden zukünftig Benutzerkonten, Rollen und
                Berechtigungen verwaltet. Die Benutzerverwaltung ist separat
                vom Mitarbeiterregister (ADR-004) und aktuell nicht implementiert.
              </div>
            </SettingsSection>
          )}

          {activeSection === "backup" && (
            <SettingsSection
              title="Backup"
              description="Sicherung und Wiederherstellung lokaler Daten"
            >
              <div className="pqm-settings-placeholder pqm-settings-placeholder--warning">
                Die Backup-Funktionalität ist aktuell nicht implementiert.
                Es werden keine Dateioperationen oder Sicherungen durchgeführt.
              </div>
            </SettingsSection>
          )}

          {activeSection === "systeminformationen" && (
            <SettingsSection
              title="Systeminformationen"
              description="Anwendungsversion und Systemarchitektur"
            >
              <div className="pqm-settings-info">
                <div className="pqm-settings-info__row">
                  <span className="pqm-settings-info__label">Version</span>
                  <span className="pqm-settings-info__value pqm-settings-info__value--mono">
                    0.9.1
                  </span>
                </div>
                <div className="pqm-settings-info__row">
                  <span className="pqm-settings-info__label">Architektur</span>
                  <span className="pqm-settings-info__value">
                    Lokale Desktop-Anwendung (Tauri, React, TypeScript, SQLite)
                  </span>
                </div>
                <div className="pqm-settings-info__row">
                  <span className="pqm-settings-info__label">Betriebsmodus</span>
                  <span className="pqm-settings-info__value">
                    Vollständiger Offline-Betrieb (ADR-002)
                  </span>
                </div>
              </div>
            </SettingsSection>
          )}
        </div>
      </div>

      <p className="pqm-einstellungen__hint">
        Hinweis: Alle Einstellungsbereiche sind Platzhalter. Es werden keine
        Konfigurationen gespeichert oder angewendet.
      </p>
    </div>
  );
}
