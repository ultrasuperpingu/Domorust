define(['app'], function (app) {
	app.controller('LogoutController', ['permissions', '$scope', '$rootScope', '$location', '$http', '$interval', function (permissions, $scope, $rootScope, $location, $http, $interval) {

		(function init() {
			var permissionList = {
				isloggedin: false,
				rights: -1,
				user: ''
			};
			permissions.setPermissions(permissionList);
			$.ajax({
				method:"DELETE",
				url: "domorust-api/login",
				async: true,
				dataType: 'json',
				success: function (data) {
					$.ajax({
						url: "domorust-api/login",
						async: false,
						dataType: 'json',
						success: function (data) {
							if (data.status === "OK") {
								if (data.user !== "") {
									permissionList.isloggedin = true;
									permissionList.user = data.user;
									permissionList.rights = parseInt(data.rights);
								}
							}
						}
					});
					permissions.setPermissions(permissionList);
					$rootScope.GetGlobalConfig();
					window.location = '#/Dashboard';
				},
				error: function (xhr, status, error) {
					var authenticate = xhr.getResponseHeader("WWW-Authenticate");
					if (authenticate && (authenticate.indexOf("Basic") > -1)) {
						// When Basic authentication is used, user should close window after logout
						$('#logout').html('<div style="text-align: center;"><span>Please close this browser tab or browser window before you log in again.</span></div>')
						return;
					}
					window.location = '#/Dashboard';
				}
			});
		})();

	}]);
});