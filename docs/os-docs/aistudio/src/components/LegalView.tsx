import React from 'react';
import { Shield, FileText, Scale } from 'lucide-react';

export function LegalView({ language = 'DE' }: { language?: 'DE' | 'EN' }) {
  const isEn = language === 'EN';

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans text-slate-300">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-slate-500/10 border border-slate-500/30 flex items-center justify-center text-slate-400 shadow-[0_0_15px_rgba(100,116,139,0.2)]">
          <Scale className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            {isEn ? "Legally Required Information" : "Gesetzlich vorgeschriebene Informationen"}
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            {isEn ? "Legal notice (Impressum) and Privacy Policy according to GDPR." : "Impressum und rechtliche Hinweise gemäß Telemediengesetz (TMG) sowie Datenschutzerklärung nach DSGVO."}
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <section className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-white/5 relative overflow-hidden group">
          <div className="flex items-center gap-3 mb-6 relative z-10">
            <FileText className="w-5 h-5 text-slate-400" />
            <h3 className="text-xl font-bold tracking-tight text-white">{isEn ? "Legal Notice (Impressum)" : "Impressum"}</h3>
          </div>
          <div className="space-y-4 text-sm font-light leading-relaxed relative z-10">
            <p>
              <strong className="font-semibold text-slate-200 block mb-1">{isEn ? "Information according to § 5 TMG" : "Angaben gemäß § 5 TMG"}</strong>
              A-TownChain GmbH<br />
              Musterstraße 123<br />
              10115 Berlin<br />
              {isEn ? "Germany" : "Deutschland"}
            </p>
            <p>
              <strong className="font-semibold text-slate-200 block mb-1">{isEn ? "Represented by:" : "Vertreten durch:"}</strong>
              {isEn ? "Managing Director" : "Geschäftsführer"}: Max Mustermann
            </p>
            <p>
              <strong className="font-semibold text-slate-200 block mb-1">{isEn ? "Contact:" : "Kontakt:"}</strong>
              {isEn ? "Phone" : "Telefon"}: +49 (0) 123 44 55 66<br />
              E-Mail: kontakt@atownchain.example.com
            </p>
            <p>
              <strong className="font-semibold text-slate-200 block mb-1">{isEn ? "Register Entry:" : "Registereintrag:"}</strong>
              {isEn ? "Entry in the commercial register." : "Eintragung im Handelsregister."}<br />
              {isEn ? "Registry court" : "Registergericht"}: Amtsgericht Berlin (Charlottenburg)<br />
              {isEn ? "Registration number" : "Registernummer"}: HRB 123456 B
            </p>
            <p>
              <strong className="font-semibold text-slate-200 block mb-1">{isEn ? "VAT ID:" : "Umsatzsteuer-ID:"}</strong>
              {isEn ? "Value added tax identification number according to § 27 a UStG:" : "Umsatzsteuer-Identifikationsnummer gemäß § 27 a Umsatzsteuergesetz:"}<br />
              DE 123456789
            </p>
          </div>
        </section>

        <section className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-white/5 relative overflow-hidden group">
          <div className="flex items-center gap-3 mb-6 relative z-10">
            <Shield className="w-5 h-5 text-slate-400" />
            <h3 className="text-xl font-bold tracking-tight text-white">{isEn ? "Privacy Policy" : "Datenschutzerklärung"}</h3>
          </div>
          <div className="space-y-4 text-sm font-light leading-relaxed relative z-10">
            <p>
              <strong>{isEn ? "1. Privacy at a Glance" : "1. Datenschutz auf einen Blick"}</strong><br />
              {isEn ? "We take the protection of your personal data very seriously. We treat your personal data confidentially and in accordance with the statutory data protection regulations and this privacy policy." : "Wir nehmen den Schutz Ihrer persönlichen Daten sehr ernst. Wir behandeln Ihre personenbezogenen Daten vertraulich und entsprechend der gesetzlichen Datenschutzvorschriften sowie dieser Datenschutzerklärung."}
            </p>
            <p>
              <strong>{isEn ? "2. Data Collection on Our Website" : "2. Datenerfassung auf unserer Website"}</strong><br />
              {isEn ? "The data processing on this website is carried out by the website operator. You can find their contact details in the legal notice. Data is collected on the one hand by you communicating it to us." : "Die Datenverarbeitung auf dieser Website erfolgt durch den Websitebetreiber. Dessen Kontaktdaten können Sie dem Impressum dieser Website entnehmen. Daten werden einerseits dadurch erhoben, dass Sie uns diese mitteilen."}
            </p>
            <p>
              <strong>{isEn ? "3. Analytics and Third-Party Tools" : "3. Analyse-Tools und Tools von Drittanbietern"}</strong><br />
              {isEn ? "When visiting our website, your surfing behavior can be statistically evaluated. This happens primarily with cookies and with so-called analysis programs." : "Beim Besuch unserer Website kann Ihr Surf-Verhalten statistisch ausgewertet werden. Das geschieht vor allem mit Cookies und mit sogenannten Analyseprogrammen."}
            </p>
            <p>
              <strong>{isEn ? "4. Rights of the Data Subject" : "4. Rechte der betroffenen Person"}</strong><br />
              {isEn ? "Within the framework of the applicable legal provisions, you have the right to free information at any time about your stored personal data, its origin and recipient, and the purpose of the data processing." : "Sie haben im Rahmen der geltenden gesetzlichen Bestimmungen jederzeit das Recht auf unentgeltliche Auskunft über Ihre gespeicherten personenbezogenen Daten, deren Herkunft und Empfänger und den Zweck der Datenverarbeitung."}
            </p>
          </div>
        </section>
      </div>
    </div>
  );
}
