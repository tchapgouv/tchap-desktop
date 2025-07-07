@echo off
setlocal enabledelayedexpansion

:: =============================================================================
:: Script d'installation de SQLCipher pour Windows
:: Basé sur le workflow GitHub Actions de Tchap Desktop
:: =============================================================================


:: vcpkg this scripvcpkgt requires visual studio build tools, install it before
:: you need to install c++ tools for windows

echo.
echo ========================================
echo  Installation de SQLCipher pour Windows
echo ========================================
echo.

:: Vérification des prérequis
echo [INFO] Vérification des prérequis...
where git >nul 2>&1
if errorlevel 1 (
    echo [ERREUR] Git n'est pas installé ou n'est pas dans le PATH
    echo Veuillez installer Git depuis https://git-scm.com/
    pause
    exit /b 1
)
echo [OK] Git est installé

:: Vérification de l'existence du dossier src-tauri
if not exist "src-tauri" (
    echo [ERREUR] Le dossier src-tauri n'existe pas
    echo Assurez-vous d'exécuter ce script depuis la racine du projet Tchap Desktop
    pause
    exit /b 1
)
echo [OK] Dossier src-tauri trouvé

:: Affichage du contenu initial de src-tauri
echo.
echo [INFO] Contenu initial du dossier src-tauri:
dir .\src-tauri\

:: Nettoyage du dossier vcpkg existant
echo.
echo [INFO] Nettoyage du dossier vcpkg existant...
if exist vcpkg (
    echo [INFO] Suppression du dossier vcpkg existant...
    rmdir /s /q vcpkg
    if errorlevel 1 (
        echo [ERREUR] Impossible de supprimer le dossier vcpkg
        pause
        exit /b 1
    )
    echo [OK] Dossier vcpkg supprimé
) else (
    echo [INFO] Aucun dossier vcpkg existant trouvé
)

:: Clonage de vcpkg
echo.
echo [INFO] Clonage du repository vcpkg...
git clone https://github.com/microsoft/vcpkg
if errorlevel 1 (
    echo [ERREUR] Échec du clonage de vcpkg
    pause
    exit /b 1
)
echo [OK] Repository vcpkg cloné avec succès

:: Changement de répertoire vers vcpkg
cd vcpkg
if errorlevel 1 (
    echo [ERREUR] Impossible d'accéder au dossier vcpkg
    pause
    exit /b 1
)

:: Bootstrap de vcpkg
echo.
echo [INFO] Initialisation de vcpkg (bootstrap)...
call .\bootstrap-vcpkg.bat -disableMetrics
if errorlevel 1 (
    echo [ERREUR] Échec de l'initialisation de vcpkg
    cd ..
    pause
    exit /b 1
)
echo [OK] vcpkg initialisé avec succès

:: Installation de SQLCipher (version statique)
echo.
echo [INFO] Installation de SQLCipher (version statique)...
call .\vcpkg.exe install sqlcipher:x64-windows-static --clean-after-build
if errorlevel 1 (
    echo [ERREUR] Échec de l'installation de SQLCipher (statique)
    cd ..
    pause
    exit /b 1
)
echo [OK] SQLCipher (statique) installé avec succès

:: Installation de SQLCipher (version dynamique)
echo.
echo [INFO] Installation de SQLCipher (version dynamique)...
call .\vcpkg.exe install sqlcipher:x64-windows --clean-after-build
if errorlevel 1 (
    echo [ERREUR] Échec de l'installation de SQLCipher (dynamique)
    cd ..
    pause
    exit /b 1
)
echo [OK] SQLCipher (dynamique) installé avec succès

:: Intégration de vcpkg
echo.
echo [INFO] Intégration de vcpkg...
call .\vcpkg.exe integrate install
if errorlevel 1 (
    echo [ERREUR] Échec de l'intégration de vcpkg
    cd ..
    pause
    exit /b 1
)
echo [OK] vcpkg intégré avec succès

:: Configuration des variables d'environnement
echo.
echo [INFO] Configuration des variables d'environnement...
set VCPKG_ROOT=%CD%
set SQLCIPHER_USE_VCPKG=1
set VCPKGRS_DYNAMIC=1
set VCPKG_DEFAULT_TRIPLET=x64-windows
set VCPKG_INSTALLED_DIR=%CD%\installed
set LIBCLANG_PATH=%CD%\installed\x64-windows\tools\libclang
set SQLCIPHER_LIB_DIR=%CD%\installed\x64-windows\lib
set SQLCIPHER_INCLUDE_DIR=%CD%\installed\x64-windows\include

echo [INFO] Variables d'environnement configurées:
echo   VCPKG_ROOT=%VCPKG_ROOT%
echo   SQLCIPHER_USE_VCPKG=%SQLCIPHER_USE_VCPKG%
echo   VCPKGRS_DYNAMIC=%VCPKGRS_DYNAMIC%
echo   VCPKG_DEFAULT_TRIPLET=%VCPKG_DEFAULT_TRIPLET%
echo   VCPKG_INSTALLED_DIR=%VCPKG_INSTALLED_DIR%
echo   LIBCLANG_PATH=%LIBCLANG_PATH%
echo   SQLCIPHER_LIB_DIR=%SQLCIPHER_LIB_DIR%
echo   SQLCIPHER_INCLUDE_DIR=%SQLCIPHER_INCLUDE_DIR%

:: Vérification des dossiers installés
echo.
echo [INFO] Contenu du dossier installed\x64-windows\lib:
if exist "installed\x64-windows\lib" (
    dir installed\x64-windows\lib
) else (
    echo [ATTENTION] Le dossier installed\x64-windows\lib n'existe pas
)

echo.
echo [INFO] Contenu du dossier installed\x64-windows-static\lib:
if exist "installed\x64-windows-static\lib" (
    dir installed\x64-windows-static\lib
) else (
    echo [ATTENTION] Le dossier installed\x64-windows-static\lib n'existe pas
)

:: Liste des packages installés
echo.
echo [INFO] Liste des packages vcpkg installés:
.\vcpkg.exe list

:: Retour au répertoire parent
cd ..

:: Copie des fichiers nécessaires vers src-tauri
echo.
echo [INFO] Copie des fichiers vers src-tauri...

:: Copie des DLL
echo [INFO] Copie de sqlcipher.dll...
if exist "vcpkg\installed\x64-windows\bin\sqlcipher.dll" (
    xcopy /y vcpkg\installed\x64-windows\bin\sqlcipher.dll .\src-tauri\
    if errorlevel 1 (
        echo [ERREUR] Échec de la copie de sqlcipher.dll
    ) else (
        echo [OK] sqlcipher.dll copié
    )
) else (
    echo [ATTENTION] sqlcipher.dll non trouvé
)

echo [INFO] Copie de libcrypto-3-x64.dll...
if exist "vcpkg\installed\x64-windows\bin\libcrypto-3-x64.dll" (
    xcopy /y vcpkg\installed\x64-windows\bin\libcrypto-3-x64.dll .\src-tauri\
    if errorlevel 1 (
        echo [ERREUR] Échec de la copie de libcrypto-3-x64.dll
    ) else (
        echo [OK] libcrypto-3-x64.dll copié
    )
) else (
    echo [ATTENTION] libcrypto-3-x64.dll non trouvé
)

echo [INFO] Copie de libssl-3-x64.dll...
if exist "vcpkg\installed\x64-windows\bin\libssl-3-x64.dll" (
    xcopy /y vcpkg\installed\x64-windows\bin\libssl-3-x64.dll .\src-tauri\
    if errorlevel 1 (
        echo [ERREUR] Échec de la copie de libssl-3-x64.dll
    ) else (
        echo [OK] libssl-3-x64.dll copié
    )
) else (
    echo [ATTENTION] libssl-3-x64.dll non trouvé
)

echo [INFO] Copie de sqlcipher.lib...
if exist "vcpkg\installed\x64-windows\lib\sqlcipher.lib" (
    xcopy /y vcpkg\installed\x64-windows\lib\sqlcipher.lib .\src-tauri\
    if errorlevel 1 (
        echo [ERREUR] Échec de la copie de sqlcipher.lib
    ) else (
        echo [OK] sqlcipher.lib copié
    )
) else (
    echo [ATTENTION] sqlcipher.lib non trouvé
)

:: Vérification finale
echo.
echo [INFO] Contenu final du dossier src-tauri:
dir .\src-tauri\

echo.
echo [INFO] Contenu du répertoire courant:
dir

echo.
echo ========================================
echo  Installation terminée avec succès !
echo ========================================
echo.
echo Les variables d'environnement suivantes ont été configurées pour cette session:
echo   VCPKG_ROOT=%VCPKG_ROOT%
echo   SQLCIPHER_USE_VCPKG=%SQLCIPHER_USE_VCPKG%
echo   VCPKGRS_DYNAMIC=%VCPKGRS_DYNAMIC%
echo   VCPKG_DEFAULT_TRIPLET=%VCPKG_DEFAULT_TRIPLET%
echo   VCPKG_INSTALLED_DIR=%VCPKG_INSTALLED_DIR%
echo   LIBCLANG_PATH=%LIBCLANG_PATH%
echo   SQLCIPHER_LIB_DIR=%SQLCIPHER_LIB_DIR%
echo   SQLCIPHER_INCLUDE_DIR=%SQLCIPHER_INCLUDE_DIR%
echo.
echo REMARQUE: Ces variables ne sont définies que pour cette session.
echo Pour les rendre permanentes, ajoutez-les manuellement aux variables
echo d'environnement système de Windows.
echo.

pause
