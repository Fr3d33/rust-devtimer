import os
import shutil

def move_files_to_individual_folders(directory):
    # Liste aller Dateien im Hauptverzeichnis (keine Unterordner)
    for filename in os.listdir(directory):
        file_path = os.path.join(directory, filename)

        # Nur Dateien (keine Ordner) bearbeiten
        if os.path.isfile(file_path):
            # Dateinamen und -endung trennen
            name, extension = os.path.splitext(filename)

            # Neuen Ordner erstellen (Name = Dateiname ohne Endung)
            new_folder_path = os.path.join(directory, name)
            os.makedirs(new_folder_path, exist_ok=True)

            # Datei in den neuen Ordner verschieben
            new_file_path = os.path.join(new_folder_path, filename)
            shutil.move(file_path, new_file_path)

            print(f"Verschoben: {filename} → {new_folder_path}/")

directory_path = r"C:\Users\furki\Downloads\Die Drei Fragezeichen"  # Hier den Pfad zu deinem Verzeichnis einfügen
move_files_to_individual_folders(directory_path)

